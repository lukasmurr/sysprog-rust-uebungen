
# hhoegl, 2018

import pickle

DATFILE = "telbuch.dat"

telbuch = {}


def getinput(prompt):
    while True:
        s = input(prompt)
        if s == "":
            pass
        else:
            if s[0] == ".":  
                return None
            else:
                return s


def save(telbuch):
    # write dict telbuch to file
    fo = open(DATFILE, "wb")
    pickle.dump(telbuch, fo)
    fo.close()



def load():
    # read dict telbuch from file
    fo = open(DATFILE, "rb")
    telbuch = pickle.load(fo)
    fo.close()
    return telbuch


def help():
    print("""usage:
! name number   # make new entry
? name          # ask for number
.               # quit
""")

    
def main():
    print("Enter 'h' or 'help' for usage.")
    while True:
        cmd = getinput("> ")
        if not cmd:
            break
        if cmd in ["help", "h"]:
            help()
            continue
        if cmd.startswith('!'):
            x, name, number = cmd.split()
            print("name:", name, ", number:", number)
            # add code ...
        elif cmd.startswith('?'):
            x, name = cmd.split()
            print("name:", name)
            # add code ...
        # define more commands, e.g. delete,
        # save, load, ...
   

if __name__ == "__main__":
    main()
