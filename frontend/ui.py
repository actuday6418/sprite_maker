import tkinterdnd2
import tkinter
from tkinterdnd2 import *
from tkinter import *
from PIL import ImageTk, Image
import libedit


def drop(event):
    entry_sv.set(event.data)

def isInt(s):
    try:
        int(s)
        return True
    except ValueError:
        return False

def doConvert():
    filename = entry.get()
    xdim = entryx.get()
    ydim = entryy.get()
    if(len(xdim) == 0 or len(ydim) == 0 or len(filename) == 0):
        print("You haven't entered every value. Please recheck your fields")
        return
    if(isInt(xdim) == False):
        print("X dim isn't an integer!")
        return
    if(isInt(ydim) == False):
        print("Y dim isn't an integer!")
        return
    libedit.edit(filename, xdim, ydim)
    print("Your sprite has been succesfully created!")



root = tkinterdnd2.Tk()
root.title("Sprite Maker")

label = tkinter.Label(text="Drop the input PNG here")
label.grid(row=0, column=0, padx=(30, 30), pady=(50, 10))

entry_sv = tkinter.StringVar()
entry = tkinter.Entry(root, textvar=entry_sv, width=80)
entry.grid(row=0, column=1, padx=(30, 30), pady=(50, 10))
entry.drop_target_register(DND_FILES)
entry.dnd_bind('<<Drop>>', drop)

labelx = tkinter.Label(text="Enter X dimension")
labelx.grid(row=1, column=0, padx=(30,30), pady=(10,10))

entry_xdim = tkinter.StringVar()
entryx = tkinter.Entry(root, textvar=entry_xdim, width=80)
entryx.grid(row=1, column=1, padx=(30,30), pady=(10,10))

labely = tkinter.Label(text="Enter Y dimension")
labely.grid(row=2, column=0, padx=(30,30), pady=(10,10))

entry_ydim = tkinter.StringVar()
entryy = tkinter.Entry(root, textvar=entry_ydim, width=80)
entryy.grid(row=2, column=1, padx=(30,30), pady=(10,10))

load = tkinter.Button(text="Convert Image", command=doConvert)
load.grid(row=3, column=1)

root.mainloop()
