using BepInEx;
using BepInEx.Logging;
using HarmonyLib;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using TMPro;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.TextCore.LowLevel;

namespace SFS_FontFix
{
    [BepInPlugin("com.sfs.fontfix", "SFS Font Fix", "5.1.0")]
    public class FontFixPlugin : BaseUnityPlugin
    {
        public static FontFixPlugin Instance;
        public static ManualLogSource Log => Instance?.Logger;

        private Font chineseUnityFont;
        private string chineseFontFilePath;
        private TMP_FontAsset chineseTMPFont;
        private bool isInitialized;
        private Font originalNormalFont;

        private void Awake()
        {
            Instance = this;
            Logger.LogInfo("SFS Font Fix v5.1.0 loaded");
            new Harmony("com.sfs.fontfix").PatchAll();
        }

        private void LateUpdate()
        {
            if (!isInitialized && SFS.Translations.TranslationManager.main != null)
                LoadAndReplaceFont();

            if (isInitialized && chineseTMPFont != null)
                ApplyTMPFontEveryFrame();
        }

        private void ApplyTMPFontEveryFrame()
        {
            foreach (var tmp in FindObjectsOfType<TMP_Text>())
            {
                if (tmp == null) continue;
                if (tmp.font != chineseTMPFont)
                {
                    tmp.font = chineseTMPFont;
                    tmp.SetAllDirty();
                    tmp.ForceMeshUpdate();
                }
                if (tmp.text != null && tmp.text.Contains("："))
                {
                    tmp.text = tmp.text.Replace('：', ':');
                    tmp.SetAllDirty();
                    tmp.ForceMeshUpdate();
                }
            }
        }

        private void LoadAndReplaceFont()
        {
            if (isInitialized) return;
            var manager = SFS.Translations.TranslationManager.main;
            if (manager?.fonts == null || manager.fonts.Count == 0) return;

            string fontPath = FindFontFile();
            if (fontPath == null) return;

            chineseUnityFont = new Font(fontPath);
            chineseFontFilePath = fontPath;

            CreateTMPFont();
            ReplaceNormalFont(manager);
            ApplyTMPFontEveryFrame();

            isInitialized = true;
        }

        private string FindFontFile()
        {
            string pluginPath = Path.GetDirectoryName(typeof(FontFixPlugin).Assembly.Location);
            foreach (var name in new[] { "NotoSansSC-Bold.ttf", "NotoSansSC-Bold.otf", "NotoSansSC.ttf", "NotoSansSC.otf", "Font.ttf", "Font.otf" })
            {
                string path = Path.Combine(pluginPath, name);
                if (File.Exists(path)) return path;
            }
            return null;
        }

        private void CreateTMPFont()
        {
            if (chineseUnityFont == null) return;
            try
            {
                chineseTMPFont = TMP_FontAsset.CreateFontAsset(
                    chineseUnityFont, 90, 9, GlyphRenderMode.SDFAA, 4096, 4096,
                    AtlasPopulationMode.Dynamic, true);
                if (chineseTMPFont == null) return;
                chineseTMPFont.name = "NotoSansSC SDF";
                chineseTMPFont.fallbackFontAssetTable = new List<TMP_FontAsset>();
            }
            catch (Exception e) { Logger.LogError("Failed to create TMP font: " + e); }
        }

        private void ReplaceNormalFont(SFS.Translations.TranslationManager manager)
        {
            var fonts = manager.fonts;
            for (int i = 0; i < fonts.Count; i++)
            {
                if (fonts[i] != null && fonts[i].name.ToLower() == "normal")
                {
                    originalNormalFont = fonts[i];
                    fonts[i] = chineseUnityFont;
                    if (manager.currentFont == originalNormalFont)
                        manager.currentFont = chineseUnityFont;
                    return;
                }
            }
            if (fonts.Count > 0 && fonts[0] != null)
            {
                originalNormalFont = fonts[0];
                fonts[0] = chineseUnityFont;
            }
        }

        public void RefreshFonts()
        {
            if (!isInitialized) return;
            var manager = SFS.Translations.TranslationManager.main;
            if (manager != null) ReplaceNormalFont(manager);
            ApplyTMPFontEveryFrame();
        }

        public static FontFixPlugin GetInstance() => Instance;
        public static bool IsReady() => Instance != null && Instance.isInitialized;
        public static TMP_FontAsset GetChineseTMPFont() => Instance?.chineseTMPFont;
        public static Font GetChineseUnityFont() => Instance?.chineseUnityFont;
    }

    [HarmonyPatch(typeof(SFS.Translations.TranslationManager), "SetLanguage")]
    public class SetLanguagePatch
    {
        [HarmonyPostfix]
        static void Postfix(SFS.Translations.TranslationManager __instance)
        {
            if (FontFixPlugin.GetChineseUnityFont() != null && __instance.currentFont != FontFixPlugin.GetChineseUnityFont())
                __instance.currentFont = FontFixPlugin.GetChineseUnityFont();
        }
    }

    [HarmonyPatch(typeof(SFS.Translations.FontSetter), "SetFont")]
    public class FontSetter_TMP_Patch
    {
        [HarmonyPostfix]
        static void Postfix(Font font, SFS.Translations.FontSetter __instance)
        {
            if (!FontFixPlugin.IsReady()) return;
            var tmp = __instance.GetComponentInChildren<TMPro.TextMeshProUGUI>();
            if (tmp != null && tmp.font != FontFixPlugin.GetChineseTMPFont())
            {
                tmp.font = FontFixPlugin.GetChineseTMPFont();
                tmp.SetAllDirty();
                tmp.ForceMeshUpdate();
            }
        }
    }

    [HarmonyPatch(typeof(SFS.World.FlightInfoDrawer), "Update")]
    public class FlightInfoDrawer_Update_Patch
    {
        [HarmonyPrefix]
        static bool Prefix(SFS.World.FlightInfoDrawer __instance)
        {
            if (!FontFixPlugin.IsReady()) return true;
            var chineseFont = FontFixPlugin.GetChineseTMPFont();
            if (chineseFont == null) return true;

            try
            {
                if (SFS.World.PlayerController.main.player.Value is SFS.World.Rocket rocket)
                {
                    __instance.menuHolder.SetActive(true);
                    float mass = rocket.rb2d.mass;
                    float thrust = rocket.partHolder.GetModules<SFS.Parts.Modules.EngineModule>()
                        .Sum((SFS.Parts.Modules.EngineModule a) => a.thrust.Value * a.throttle_Out.Value);

                    __instance.massText.Text = mass.ToString("0.00") + " t";
                    __instance.thrustText.Text = thrust.ToString("0.0") + " kN";
                    __instance.thrustToWeightText.Text = (mass > 0 ? (thrust / mass).ToString("0.00") : "0.00");
                    __instance.partCountText.Text = rocket.partHolder.parts.Count.ToString();
                }
                else
                {
                    __instance.massText.Text = "0.00 t";
                    __instance.thrustText.Text = "0.0 kN";
                    __instance.thrustToWeightText.Text = "0.00";
                    __instance.partCountText.Text = "0";
                }
                __instance.timewarpText.Text = SFS.World.WorldTime.main.timewarpSpeed + "x";
            }
            catch { }

            ForceRefresh(__instance.timewarpText, chineseFont);
            ForceRefresh(__instance.massText, chineseFont);
            ForceRefresh(__instance.thrustText, chineseFont);
            ForceRefresh(__instance.thrustToWeightText, chineseFont);
            ForceRefresh(__instance.partCountText, chineseFont);
            return false;
        }

        static void ForceRefresh(SFS.UI.TextAdapter adapter, TMP_FontAsset font)
        {
            if (adapter == null) return;
            var tmp = adapter.GetComponent<TMPro.TextMeshProUGUI>();
            if (tmp == null) return;
            if (tmp.font != font) tmp.font = font;
            if (tmp.text != null && tmp.text.Contains("："))
                tmp.text = tmp.text.Replace('：', ':');
            tmp.SetAllDirty();
            tmp.ForceMeshUpdate();
        }
    }
}
