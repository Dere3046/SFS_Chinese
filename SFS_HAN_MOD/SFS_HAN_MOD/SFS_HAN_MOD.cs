using HarmonyLib;
using ModLoader;
using SFS.UI.ModGUI;
using SFS.World;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using TMPro;
using UnityEngine;
using UnityEngine.TextCore.LowLevel;

namespace SFS_HAN_MOD
{
    public class SFS_HAN_MOD : Mod
    {
        public static SFS_HAN_MOD Instance;
        private FontFixHelper helper;
        static void Log(string msg) => UnityEngine.Debug.Log("[SFS_HAN_MOD] " + msg);
        static void Log(object msg) => UnityEngine.Debug.Log("[SFS_HAN_MOD] " + msg);

        public override string ModNameID => "sfs_font_fix";
        public override string DisplayName => "SFS Font Fix";
        public override string Author => "Dere3046";
        public override string ModVersion => "5.1.0";
        public override string MinimumGameVersionNecessary => "1.5.9";
        public override string Description => "Fixes missing Chinese characters by replacing the normal font with Noto Sans SC";

        public Font chineseUnityFont;
        public TMP_FontAsset chineseTMPFont;
        public bool isInitialized;

        public override void Early_Load()
        {
            Instance = this;
            new Harmony("com.sfs.fontfix_native").PatchAll();
        }

        public override void Load()
        {
            var go = GameObject.Find("FontFixHelper");
            if (go != null) GameObject.Destroy(go);

            go = new GameObject("FontFixHelper");
            helper = go.AddComponent<FontFixHelper>();
            helper.mod = this;
            GameObject.DontDestroyOnLoad(go);

            TryReplaceFont();
        }

        public void TryReplaceFont()
        {
            if (isInitialized) return;
            var manager = SFS.Translations.TranslationManager.main;
            if (manager?.fonts == null || manager.fonts.Count == 0) return;

            string fontPath = FindFontFile();
            if (fontPath == null) return;

            chineseUnityFont = new Font(fontPath);
            ReplaceNormalFont(manager);
            CreateTMPFont();
            isInitialized = true;
            Log("Font replacement complete");
        }

        private string FindFontFile()
        {
            string modDir = Path.GetDirectoryName(typeof(SFS_HAN_MOD).Assembly.Location);
            foreach (var name in new[] { "NotoSansSC-Bold.ttf", "NotoSansSC.ttf", "Font.ttf" })
            {
                string path = Path.Combine(modDir, name);
                if (File.Exists(path)) return path;
            }
            string parentDir = Path.GetDirectoryName(modDir);
            if (parentDir != null)
            {
                foreach (var name in new[] { "NotoSansSC-Bold.ttf", "NotoSansSC.ttf", "Font.ttf" })
                {
                    string path = Path.Combine(parentDir, name);
                    if (File.Exists(path)) return path;
                }
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
            catch (System.Exception e) { Log("CreateFontAsset error: " + e.Message); }
        }

        private void ReplaceNormalFont(SFS.Translations.TranslationManager manager)
        {
            var fonts = manager.fonts;
            for (int i = 0; i < fonts.Count; i++)
            {
                if (fonts[i] != null && fonts[i].name.ToLower() == "normal")
                {
                    fonts[i] = chineseUnityFont;
                    if (manager.currentFont == fonts[i])
                        manager.currentFont = chineseUnityFont;
                    return;
                }
            }
            if (fonts.Count > 0 && fonts[0] != null)
                fonts[0] = chineseUnityFont;
        }

        public void ApplyFontToAllTMP()
        {
            if (chineseTMPFont == null) return;
            int count = 0;
            foreach (var tmp in UnityEngine.Object.FindObjectsOfType<TMP_Text>())
            {
                if (tmp == null) continue;
                tmp.font = chineseTMPFont;
                tmp.SetAllDirty();
                tmp.ForceMeshUpdate();
                count++;
            }
        }
    }

    public class FontFixHelper : MonoBehaviour
    {
        public SFS_HAN_MOD mod;

        void Update()
        {
            if (mod == null || mod.isInitialized) return;
            mod.TryReplaceFont();
        }

        void LateUpdate()
        {
            if (mod == null || !mod.isInitialized || mod.chineseTMPFont == null) return;
            mod.ApplyFontToAllTMP();
        }
    }

    [HarmonyPatch(typeof(SFS.Translations.TranslationManager), "SetLanguage")]
    public class SetLanguagePatch
    {
        [HarmonyPostfix]
        static void Postfix(SFS.Translations.TranslationManager __instance)
        {
            var mod = SFS_HAN_MOD.Instance;
            if (mod?.chineseUnityFont != null && __instance.currentFont != mod.chineseUnityFont)
                __instance.currentFont = mod.chineseUnityFont;
        }
    }

    [HarmonyPatch(typeof(SFS.Translations.FontSetter), "SetFont")]
    public class FontSetter_TMP_Patch
    {
        [HarmonyPostfix]
        static void Postfix(Font font, SFS.Translations.FontSetter __instance)
        {
            var mod = SFS_HAN_MOD.Instance;
            if (mod?.chineseTMPFont == null) return;
            var tmp = __instance.GetComponentInChildren<TextMeshProUGUI>();
            if (tmp != null && tmp.font != mod.chineseTMPFont)
            {
                tmp.font = mod.chineseTMPFont;
                tmp.SetAllDirty();
                tmp.ForceMeshUpdate();
            }
        }
    }

    [HarmonyPatch(typeof(SFS.World.FlightInfoDrawer), "Update")]
    public class FlightInfoDrawer_Update_Patch
    {
        [HarmonyPrefix]
        static bool Prefix(FlightInfoDrawer __instance)
        {
            var mod = SFS_HAN_MOD.Instance;
            if (mod?.chineseTMPFont == null) return true;

            try
            {
                if (PlayerController.main.player.Value is SFS.World.Rocket rocket)
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
                __instance.timewarpText.Text = WorldTime.main.timewarpSpeed + "x";
            }
            catch { }

            ForceRefresh(__instance.timewarpText, mod.chineseTMPFont);
            ForceRefresh(__instance.massText, mod.chineseTMPFont);
            ForceRefresh(__instance.thrustText, mod.chineseTMPFont);
            ForceRefresh(__instance.thrustToWeightText, mod.chineseTMPFont);
            ForceRefresh(__instance.partCountText, mod.chineseTMPFont);
            return false;
        }

        static void ForceRefresh(SFS.UI.TextAdapter adapter, TMP_FontAsset font)
        {
            if (adapter == null) return;
            var tmp = adapter.GetComponent<TextMeshProUGUI>();
            if (tmp == null) return;
            if (tmp.font != font) tmp.font = font;
            if (tmp.text != null && tmp.text.Contains("："))
                tmp.text = tmp.text.Replace('：', ':');
            tmp.SetAllDirty();
            tmp.ForceMeshUpdate();
        }
    }
}
