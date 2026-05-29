import tkinter as tk
from tkinter import filedialog, ttk, messagebox
from pathlib import Path
# 配置文件
config = {
     "Theme": "",
     "GamePath": ""
}
themes = {
    "black": {
        "bg": "#2b2b2b",
        "fg": "#ffffff",
        "button_bg": "#1E74C4",
        "button_active_bg": "#1669B0", # 鼠标悬停时的颜色（可选）
    },
    "white": {
        "bg": "#bebebe",
        "fg": "#000000",
        "button_bg": "#1E74C4",
        "button_active_bg": "#1669B0",
    }
}

def open_config():
    try:
        with open("config.txt", "r", encoding="utf-8") as f:
                line = f.readline()
                config["Theme"] = line.strip() or "black"
                
                line = f.readline()
                if line:
                    path = Path(line) / "Spaceflight Simulator.exe"

                    if path.is_file():
                        config["GamePath"] = line.strip()
                else:
                    config["GamePath"] = ""
                    
                print(f"主题:{config['Theme']}\n游戏路径:{config['GamePath'] or '未选择'}")
    except FileNotFoundError:
         print("警告:未找到配置文件!")
         config["Theme"] = "black"

def write_config():
    try:
        with open("config.txt", "w", encoding="utf-8") as f:
            f.write(config["Theme"] + "\n" + config["GamePath"])
    except Exception:
        print("权限不足")

open_config()

# 获取当前主题配置
current_theme_name = config.get("Theme", "black")
theme = themes[current_theme_name]

# 初始化变量
button_normal_state = tk.NORMAL
button_disabled_state = tk.DISABLED
button_relief = tk.FLAT
# 初始化窗口
root = tk.Tk()
root.title("SFS汉化工具")
root.geometry("700x600")
root.minsize(700, 600)
root.maxsize(700, 600)
root.config(bg=theme["bg"])

# 功能区
class Features():
    """按钮等功能区"""
    def assign(self, number):
        """功能选择区"""
        if number == 1:
             self.select_game()
    def select_game(self):
        """选择游戏根目录文件夹"""
        # 打开选择目录文件夹
        game_dir = filedialog.askdirectory(title=r"选择游戏根目录(例如: \Steam..\游戏名)")
        if game_dir:
            print("游戏目录:" + game_dir)
            path = Path(game_dir) / "Spaceflight Simulator.exe"
            if path.is_file():
                  config["GamePath"] = game_dir
                  Button_select_gamepath.destroy()
                  Label_select_gamepath.destroy()
                  write_config()
            else:
             print("警告:这不是一个有效的路径")
                  

features = Features()

# 主题下拉菜单
theme_var = tk.StringVar(value=current_theme_name)

# 创建TK.OPTionmenu
# 参数依次是: 父组件, 关联变量, 默认值, *选项列表
theme_menu = tk.OptionMenu(
    root,
    theme_var,
    *themes.keys()
)
theme_menu.config(
    bg=theme["button_bg"],
    fg=theme["fg"],
    highlightthickness=0
)

# 定义控件
Button_select_gamepath = tk.Button(
     root, 
     text="手动选择游戏根目录（自动寻找功能开发ing..）", 
     bg=theme["button_bg"],
     fg=theme["fg"],
     relief=button_relief,
     command=lambda: features.assign(1)
)
Label_select_gamepath = tk.Label(
     root,
     text=r"例如: \Steam..\游戏名",
     bg=theme["bg"],
     fg=theme["fg"]
)

#  UI界面 
def GUI():  
    """ UI界面"""
    global Button_select_gamepath, Label_select_gamepath
    if not config["GamePath"]:
        Button_select_gamepath.grid(column=0, row=0, padx=(5, 5), pady=(5, 5))
        Label_select_gamepath.grid(column=0, row=1, padx=(5, 5), pady=(5, 5))

if __name__ == "__main__":
    GUI()
    root.mainloop()
