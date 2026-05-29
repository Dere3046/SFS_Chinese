using BepInEx;
using BepInEx.Logging;
using HarmonyLib;
using System;
using System.Collections.Generic;
using System.IO;
using TMPro;
using UnityEngine;
using UnityEngine.TextCore.LowLevel;

namespace SFS_FontFix
{
    [BepInPlugin("com.sfs.fontfix", "SFS Font Fix", "5.1.0")]
    public class FontFixPlugin : BaseUnityPlugin
    {
        public static FontFixPlugin Instance;
        public static ManualLogSource Log => Instance?.Logger;

        public Font chineseUnityFont;
        public TMP_FontAsset chineseTMPFont;
        public bool isInitialized;

        private void Awake()
        {
            Instance = this;
            Logger.LogInfo("SFS Font Fix v5.1.0 loaded");
            new Harmony("com.sfs.fontfix").PatchAll();
        }

        private void Start()
        {
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
            ApplyFontToExistingTMP();

            isInitialized = true;
            Logger.LogInfo("Font replacement complete");
        }

        private string FindFontFile()
        {
            string pluginPath = Path.GetDirectoryName(typeof(FontFixPlugin).Assembly.Location);
            foreach (var name in new[] { "NotoSansSC-Bold.ttf", "NotoSansSC.ttf", "Font.ttf" })
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
            catch (Exception e) { Logger.LogError("CreateFontAsset error: " + e); }
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

        private void ApplyFontToExistingTMP()
        {
            if (chineseTMPFont == null) return;
            var tmps = UnityEngine.Object.FindObjectsByType<TextMeshProUGUI>(FindObjectsSortMode.None);
            foreach (var tmp in tmps)
            {
                if (tmp != null)
                    tmp.font = chineseTMPFont;
            }
            Logger.LogInfo("Applied font to " + tmps.Length + " existing TMP_Text objects");
        }

        public static void ApplyFontToTMP(TextMeshProUGUI tmp)
        {
            var mod = Instance;
            if (mod?.chineseTMPFont == null) return;
            if (tmp.font != mod.chineseTMPFont)
                tmp.font = mod.chineseTMPFont;
        }
    }

    [HarmonyPatch(typeof(SFS.Translations.TranslationManager), "Awake")]
    public class TranslationManager_Awake_Patch
    {
        [HarmonyPostfix]
        static void Postfix(SFS.Translations.TranslationManager __instance)
        {
            var mod = FontFixPlugin.Instance;
            if (mod == null) return;
            mod.TryReplaceFont();
        }
    }

    [HarmonyPatch(typeof(SFS.Translations.TranslationManager), "SetLanguage")]
    public class SetLanguagePatch
    {
        [HarmonyPostfix]
        static void Postfix(SFS.Translations.TranslationManager __instance)
        {
            if (FontFixPlugin.Instance?.chineseUnityFont != null && __instance.currentFont != FontFixPlugin.Instance.chineseUnityFont)
                __instance.currentFont = FontFixPlugin.Instance.chineseUnityFont;
        }
    }

    [HarmonyPatch(typeof(SFS.Translations.FontSetter), "SetFont")]
    public class FontSetter_SetFont_Patch
    {
        [HarmonyPostfix]
        static void Postfix(SFS.Translations.FontSetter __instance)
        {
            if (FontFixPlugin.Instance?.chineseTMPFont == null) return;
            var tmp = __instance.GetComponentInChildren<TextMeshProUGUI>();
            if (tmp != null && tmp.font != FontFixPlugin.Instance.chineseTMPFont)
                tmp.font = FontFixPlugin.Instance.chineseTMPFont;
        }
    }

    [HarmonyPatch(typeof(SFS.UI.TextAdapter), "Text", MethodType.Setter)]
    public class TextAdapter_TextSetter_Patch
    {
        [HarmonyPostfix]
        static void Postfix(SFS.UI.TextAdapter __instance)
        {
            if (FontFixPlugin.Instance?.chineseTMPFont == null) return;
            var tmp = __instance.GetComponent<TextMeshProUGUI>();
            if (tmp != null && tmp.font != FontFixPlugin.Instance.chineseTMPFont)
                tmp.font = FontFixPlugin.Instance.chineseTMPFont;
        }
    }

    [HarmonyPatch(typeof(TextMeshProUGUI), "OnEnable")]
    public class TMP_OnEnable_Patch
    {
        [HarmonyPostfix]
        static void Postfix(TextMeshProUGUI __instance)
        {
            FontFixPlugin.ApplyFontToTMP(__instance);
        }
    }
}
