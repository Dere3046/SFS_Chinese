try:
    import tkinter as tk
    from tkinter import messagebox as mb, filedialog as fdl, ttk
    import os
    import zipfile
    import urllib.request
    import urllib.error
except ImportError:
    print("error")

# 初始化
Toolroot = tk.Tk()
Toolroot.title("SFS文本编辑工具")
Toolroot.geometry("800x500")
Toolroot.maxsize(800, 500)
Toolroot.minsize(700, 400)
Toolroot.config(bg="#101010")
Toolroot.columnconfigure(1, weight=1)
GameDir, LanguageSettings_2_Path, LanguageDir, ModDir = ""
Font = ("黑体", 14)
bg = "#2F2F2F"
green_fg = "green"
red_fg = "red"
fg = "white"
MODurl = "https://release-assets.githubusercontent.com/github-production-release-asset/1207327489/9b2d605f-51ed-4c55-9b23-57e3bd2840c7?sp=r&sv=2018-11-09&sr=b&spr=https&se=2026-04-12T02%3A53%3A03Z&rscd=attachment%3B+filename%3DSFS_HAN_MODv5.1.0.zip&rsct=application%2Foctet-stream&skoid=96c2d410-5711-43a1-aedd-ab1947aa7ab0&sktid=398a6654-997b-47e9-b12b-9515b896b4de&skt=2026-04-12T01%3A52%3A59Z&ske=2026-04-12T02%3A53%3A03Z&sks=b&skv=2018-11-09&sig=20woqBqhIiS8giMhN0rHMolCh%2BxIL%2BHGbGS8hlV2SXY%3D&jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmVsZWFzZS1hc3NldHMuZ2l0aHVidXNlcmNvbnRlbnQuY29tIiwia2V5Ijoia2V5MSIsImV4cCI6MTc3NTk1OTg0NywibmJmIjoxNzc1OTU5NTQ3LCJwYXRoIjoicmVsZWFzZWFzc2V0cHJvZHVjdGlvbi5ibG9iLmNvcmUud2luZG93cy5uZXQifQ.8cK3N6IfeOxcb0p_gDrWno0uYyO0IObpMXKvSGZ7zcE&response-content-disposition=attachment%3B%20filename%3DSFS_HAN_MODv5.1.0.zip&response-content-type=application%2Foctet-stream"
Elements = []
Texts = [
    "1.选择游戏根目录(否则软件无法使用)",
    "选择的文件夹不是游戏根目录,请重新选择",
    "检查中",
    "未安装字体修复MOD",
    "已安装字体修复MOD",
    "字体修复MOD:",
    "汉化及自定义文字:",
    "使用自定义汉化文件"
]
Buttons = [
    "点击选择游戏根目录",
    "安装MOD",
    "一键汉化"
]

# 定义下载
def DownloadMOD(text):
    try:
        urllib.request.urlretrieve(MODurl, os.path.join(os.path.dirname(os.path.abspath(__file__)), "SFS_HAN_MODv5.1.0.zip"))
        text.config(text="下载完成", fg=green_fg)
    except urllib.error.URLError:
        text.config(text="下载失败（网络）", fg=red_fg)
    except Exception:
        text.config(text="下载失败（其他）", fg=red_fg)
                
# 定义寻找自定义文件
def FoundTextFile():
    File = os.listdir(LanguageDir)
    File.remove("Example.txt")
    File.append("请选择")
    return File

# 安装MOD
def StepMOD(Button, text):
    if not os.path.exists(ModDir):
        os.makedirs(ModDir)
    try:    
        with zipfile.ZipFile(os.path.join(os.path.dirname(os.path.abspath(__file__)), "SFS_HAN_MODv5.1.0.zip"), "r") as zip:
            zip.extractall(ModDir)
            Button.destroy()
            text.config(text=Texts[4], fg=green_fg)
    except FileNotFoundError:
        mb.showerror("错误", "MOD已丢失")
        result = mb.askyesno
        if result:
            DownloadMOD(text)

# 获取用户选择的自定义文件
def GetFile(Frame, Combobox):
    selected = Combobox.get()
    text = tk.Label(Frame, text="未选择文件", fg=red_fg, font=Font, bg=bg)
    text.grid(sticky="w", row=3, column=0, padx=5, pady=(3, 5))
    Elements.append(text)
    if selected == "请选择":
        pass
    else:
        text.config(text=f"更改成功，已选择:{selected}", fg=green_fg)
        with open(LanguageSettings_2_Path, "w", encoding="utf-8") as f:
            f.writelines("{")
            f.writelines(f" \"codeName\": \"{selected}\",")
            f.writelines(" \"custom\": true")
            f.writelines("}")

# 定义一键汉化
def Chines():
    with open(LanguageSettings_2_Path, "w", encoding="utf-8") as f:
        f.writelines("{")
        f.writelines(" \"codeName\": \"CN\",")
        f.writelines(" \"custom\": true")
        f.writelines("}")
    try:    
        with zipfile.ZipFile("SFS_Chinese.zip", "r") as zf:
            zf.extractall(LanguageDir)
            mb.showinfo("提示", "成功汉化")
    except FileNotFoundError:
        mb.showerror("错误", "未找到汉化文件")
# 定义安装字体修复MOD的功能
def ManageMOD():
    Frame1 = tk.Frame(Toolroot, bg=bg, relief=tk.RAISED, borderwidth=1)
    Text1 = tk.Label(Frame1, text=Texts[5], fg=fg, bg=bg, font=Font)
    Text2 = tk.Label(Frame1, text=Texts[2], bg=bg, fg=fg, font=Font)
    Frame1.grid(row=1, column=1, sticky="w", padx=30, pady=10, ipady=1)
    Text2.grid(row=1, column=0, sticky="w", pady=(10, 5))
    Text1.grid(row=0, column=0, padx=(5, 5), pady=(3, 3))
    Elements.append(Text1)
    Elements.append(Frame1)
    Elements.append(Text2)
    if not os.path.exists(os.path.join(ModDir, "SFS_HAN_MOD")):
        Text2.config(text=Texts[3], fg=red_fg)
        Button2 = tk.Button(Frame1, text=Buttons[1], fg=fg, bg=bg, font=Font, command=lambda:StepMOD(Button2, Text2))
        Elements.append(Button2)
        Button2.grid(row=2, column=0, sticky="w", pady=(2, 5), padx=(5,5))
    elif os.path.exists(os.path.join(ModDir, "SFS_HAN_MOD")):
        Text2.config(text=Texts[4], fg=green_fg)

# 定义设置自定义及汉化文字（LanguageSettings_2.txt）
def SetForLanguageSetting_2GUI():
    Frame2 = tk.Frame(Toolroot, bg=bg, relief=tk.RAISED, borderwidth=0.5)
    Text3 = tk.Label(Frame2, text=Texts[6], bg=bg, fg=fg, font=Font)
    Button3 = tk.Button(Frame2, text=Buttons[2], font=Font, fg=fg, bg=bg, command=Chines)
    Text4 = tk.Label(Frame2, text=Texts[7], bg=bg, fg=fg, font=Font)
    File = FoundTextFile()
    Combobox = ttk.Combobox(Frame2, values=File)
    Combobox.current(len(File) - 1)
    Button4 = tk.Button(Frame2, text=Buttons[3], font=Font, fg=fg, bg=bg, command=lambda:GetFile(Frame2, Combobox))
    Frame2.grid(row=2, column=2, sticky="e", padx=40, pady=10, ipadx=(5, 5), ipady=(3, 3))
    Text3.grid(sticky="w", row=0, column=0, padx=5, pady=(3, 5))
    Button3.grid(sticky="w", row=1, column=0, padx=5, pady=(3, 5))
    Text4.grid(sticky="w", row=2, column=0, padx=5, pady=(6, 5))
    Combobox(sticky="w", row=4, column=0, padx=5, pady=(3, 5))
    Button4.grid(sticky="w", row=5, column=0, padx=5, pady=(3, 5))
    Elements.append(Frame2)
    Elements.append(Text4)
    Elements.append(Text3)
    Elements.append(Button3)
    Elements.append(Button4)
    Elements.append(Combobox)
# 定义选择游戏目录指令
def SeletGameDir():
    global GameDir, LanguageSettings_2_Path, LanguageDir, ModDir
    path = fdl.askdirectory(title="选择游戏根目录")
    if path:
        if os.path.isfile(os.path.join(path, r"Spaceflight Simulator.exe")):
            GameDir = path
            LanguageSettings_2_Path = os.path.join(GameDir, r"Saving\Settings\LangSettings_2.txt")
            LanguageDir = os.path.join(GameDir, r"Spaceflight Simulator_Data\Custom Translations")
            ModDir = os.path.join(GameDir, "Mods")
            for w in Elements:
                w.destroy()
            Text1 = tk.Label(Frame, bg=bg, fg=green_fg, font=Font, text=f"已选择游戏目录：{GameDir}")
            Text1.grid(row=0, column=0)
            Elements.append(Text1)
            ManageMOD()
        else:
            Text.config(text=Texts[1], fg=red_fg)

Frame = tk.Frame(Toolroot, borderwidth=1, relief=tk.RAISED, bg=bg)
Frame.grid(row=0,column=1, sticky="w", ipadx=5, ipady=5, padx=10, pady=10)
Text = tk.Label(Frame, bg=bg, font=Font, text=Texts[0], fg=fg)
Button = tk.Button(Frame, bg=bg, font=Font, text=Buttons[0], fg=fg, command=SeletGameDir)
Text.grid(row=0, column=0)
Button.grid(row=1, column=0)
Elements.append(Text)
Elements.append(Button)

Toolroot.mainloop()