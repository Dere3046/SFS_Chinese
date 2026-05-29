using HarmonyLib;
using ModLoader;
using System;
using System.Collections.Generic;
using System.IO;
using TMPro;
using UnityEngine;
using UnityEngine.TextCore.LowLevel;

namespace SFS_HAN_MOD
{
    public class SFS_HAN_MOD : Mod
    {
        public static SFS_HAN_MOD Instance;
        static void Log(string msg) => UnityEngine.Debug.Log("[SFS_HAN_MOD] " + msg);

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
            TryReplaceFont();
            if (!isInitialized)
            {
                var go = new GameObject("FontFixInit");
                go.AddComponent<FontFixRetry>().mod = this;
                GameObject.DontDestroyOnLoad(go);
            }
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
            catch (Exception e) { Log("CreateFontAsset error: " + e.Message); }
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
            Log("Applied font to " + tmps.Length + " existing TMP_Text objects");
        }

        public static void ApplyFontToTMP(TextMeshProUGUI tmp)
        {
            var mod = Instance;
            if (mod?.chineseTMPFont == null) return;
            if (tmp.font != mod.chineseTMPFont)
                tmp.font = mod.chineseTMPFont;
        }
    }

    public class FontFixRetry : MonoBehaviour
    {
        public SFS_HAN_MOD mod;

        void Update()
        {
            if (mod.isInitialized)
            {
                Destroy(gameObject);
                return;
            }
            mod.TryReplaceFont();
        }
    }

    [HarmonyPatch(typeof(SFS.Translations.TranslationManager), "Awake")]
    public class TranslationManager_Awake_Patch
    {
        [HarmonyPostfix]
        static void Postfix(SFS.Translations.TranslationManager __instance)
        {
            var mod = SFS_HAN_MOD.Instance;
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
            var mod = SFS_HAN_MOD.Instance;
            if (mod?.chineseUnityFont != null && __instance.currentFont != mod.chineseUnityFont)
                __instance.currentFont = mod.chineseUnityFont;
        }
    }

    [HarmonyPatch(typeof(SFS.Translations.FontSetter), "SetFont")]
    public class FontSetter_SetFont_Patch
    {
        [HarmonyPostfix]
        static void Postfix(SFS.Translations.FontSetter __instance)
        {
            var mod = SFS_HAN_MOD.Instance;
            if (mod?.chineseTMPFont == null) return;
            var tmp = __instance.GetComponentInChildren<TextMeshProUGUI>();
            if (tmp != null && tmp.font != mod.chineseTMPFont)
                tmp.font = mod.chineseTMPFont;
        }
    }

    [HarmonyPatch(typeof(TextMeshProUGUI), "OnEnable")]
    public class TMP_OnEnable_Patch
    {
        [HarmonyPostfix]
        static void Postfix(TextMeshProUGUI __instance)
        {
            SFS_HAN_MOD.ApplyFontToTMP(__instance);
        }
    }
}
