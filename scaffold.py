import sys
import requests
import os
import pyperclip

def scaffold_rust(year, day):
    RUST_TEMPLATE = "rust_template.rs"

    # make directory for year
    try:
        os.mkdir(year + "-advent-of-code")
    except FileExistsError:
        pass

    os.chdir(year + "-advent-of-code")

    # Cargo init day{day}
    os.system(f"cargo init day{day}")

    # remove default main.rs
    os.remove(f"day{day}/src/main.rs")

    # copy template to day{day}/src/main.rs
    os.system(f"cp ../{RUST_TEMPLATE} day{day}/src/main.rs")

    # grab example input
    input = grab_input(year, day, "../")

    # write input to day{day}/input.txt
    with open(f"day{day}/src/input.txt", "w") as f:
        f.write(input)

    # check clipboard for example input
    clipboard = pyperclip.paste()

    if clipboard == None:
        clipboard = "Nothing in clipboard"

    with open(f"day{day}/src/example_input.txt", "w") as f:
        f.write(clipboard)

    # change users current directory to day{day}
    os.chdir(f"day{day}/src")


def grab_input(year, day, prepend_path=""):
    session = open(f"{prepend_path}.session", "r").read().strip()
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    cookies = {"session": session}
    response = requests.get(url, cookies=cookies)
    if response.status_code == 200:
        return response.text
    else:
        print("Error, invalid session cookie")
        sys.exit(1)

def main():
    # command layout
    # scaffold.py <year> <day> <language>
    
    if len(sys.argv) < 4:
        print("Usage: scaffold.py <year> <day> <language>")
        sys.exit(1)

    year = sys.argv[1]
    day = sys.argv[2]
    language = sys.argv[3]

    if language == "rust":
        scaffold_rust(year, day)
    else:
        print("Language not supported")
        sys.exit(1)

if __name__ == "__main__":
    main()
