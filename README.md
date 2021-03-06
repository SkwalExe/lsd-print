# lsd-printπ§ͺ

![](images/1.png)

π§ͺ Just a print tool, but we gave it lsd

# Installation π¦

## Arch Linux π§

lsd-print is in the AUR

```bash
yay -S lsd-print
```

## Other πͺπ§

### With make - Linux π§

Run make

```bash
# π lsd-print/
make
```

### Build from source - Linux π§ & Windows πͺ

**Clone this repo**

```bash
git clone https://github.com/SkwalExe/lsd-print.git
```

build with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

```bash
# π lsd-print/
cargo build --release
```

**[ LINUX ONLY ] :** Move the binary

```bash
# π lsd-print/
sudo cp target/release/lsd-print /usr/bin/lsd-print
```

**On windows** the executable will be `target\release\lsd-print.exe` you can move it wherever you want.

# Usage π

![](images/2.png)

## Example 

```bash
la | lsd-print -b 
``` 

![](images/3.png)

# Docker π³

## Run the latest version

```bash
docker run --rm -it ghcr.io/skwalexe/lsd-print:main
```

## Test your changes π§

### Build π οΈ

```bash
# π lsd-print/
docker build -t lsd-print .
```

### Run π

```bash
docker run --rm -it lsd-print [OPTIONS]
```

# Uninstall π

## With make

Run make uninstall

```bash
# π help-ukraine/
make uninstall
```

## Or

Just remove the binary

```bash
sudo rm /usr/bin/lsd-print
```

# final

If you have any problem, don't hesitate to open an issue

# contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

<a href="https://github.com/SkwalExe#ukraine"><img src="https://raw.githubusercontent.com/SkwalExe/SkwalExe/main/ukraine.jpg" width="100%" height="15px" /></a>