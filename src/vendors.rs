use eyre::{eyre, Result};
use std::process::Command;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vendor {
    Apk,
    Apt,
    Brew,
    Cards,
    Choco,
    Dnf,
    Emerge,
    Eopkg,
    Flatpak,
    Guix,
    NixEnv,
    Opkg,
    Pacman,
    Pkg,
    Pkgman,
    Ports,
    Scoop,
    Slackpkg,
    Snap,
    Urpm,
    Termux,
    Winget,
    Xbps,
    Yay,
    Yum,
    Zypper,
}

#[derive(Debug, Clone, Copy)]
pub enum PlsCommand {
    Install,
    Remove,
    Upgrade,
    Search,
    Info,
    Update,
    UpgradeAll,
    List,
}

impl Vendor {
    pub fn new() -> Result<Self> {
        let vendors = get_vendors();
        for vendor in vendors.iter() {
            let vendor_data: VendorData = (*vendor).into();
            if let Ok(_) = which::which(vendor_data.1[0]) {
                return Ok(*vendor)
            }
        }
        Err(eyre!(
            "no vendor installed, candidates are: {}",
            vendors.iter().map(|vendor| format!("{:?}", vendor)).collect::<Vec<String>>().join(", "),
        ))
    }

    pub fn execute(self, command: PlsCommand, args: &str, yes: bool, dry_run: bool) -> Result<i32> {
        let vendor_data: VendorData = self.into();
        let command = command.format(vendor_data, args, yes);

        if command.is_empty() {
            eprintln!("command not supported by the current vendor");
            return Ok(1)
        }

        if dry_run {
            eprintln!("{}", command);
            return Ok(0);
        }

        #[cfg(target_os = "windows")]
        let status = Command::new("cmd").args(["/C", &command]).status()?;
        #[cfg(not(target_os = "windows"))]
        let status = Command::new("sh").args(["-c", &command]).status()?;

        Ok(status.code().unwrap_or_default())
    }
}

impl PlsCommand {
    fn format(self, vendor: VendorData, args: &str, yes: bool) -> String {
        match self {
            PlsCommand::Install => vendor.1[2].to_owned(),
            PlsCommand::Remove => vendor.1[3].to_owned(),
            PlsCommand::Upgrade => vendor.1[4].to_owned(),
            PlsCommand::Search => vendor.1[5].to_owned(),
            PlsCommand::Info => vendor.1[6].to_owned(),
            PlsCommand::Update => vendor.1[7].to_owned(),
            PlsCommand::UpgradeAll => vendor.1[8].to_owned(),
            PlsCommand::List => vendor.1[9].to_owned(),
        }
            .replace("$yes", if yes {vendor.1[1]} else {""})
            .replace("$args", args)
    }
}


//----------------------------------------------------------------------------//
use Vendor::*;

fn get_vendors() -> Vec<Vendor> {
    vec![
        #[cfg(target_os = "android")]
        Termux,
        #[cfg(target_os = "haiku")]
        Pkgman,
        #[cfg(target_os = "linux")] /* TODO: filter by distro */
        Apt, Dnf, Yum, Yay, Pacman, Apk, Emerge, Slackpkg, Zypper, NixEnv, Xbps, Cards, Urpm, Eopkg, Flatpak, Snap,
        #[cfg(target_os = "macos")]
        Brew, Ports,
        #[cfg(target_os = "windows")]
        Scoop, Choco, Winget,
    ]
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct VendorData(Vendor, [&'static str; 10]);

static VENDORS: [VendorData; 26] = [
    VendorData(Apk, [
        "apk",
        "",
        "apk add $args",
        "apk del $args",
        "apk upgrade $args",
        "apk search $args",
        "apk info $args",
        "apk update",
        "apk upgrade",
        "apk list --installed",
    ]),
    VendorData(Apt, [
        "apt",
        "--yes",
        "apt install $yes $args",
        "apt remove $yes $args",
        "apt install --only-upgrade $yes $args",
        "apt search $args",
        "apt show $args",
        "apt update $yes",
        "apt upgrade $yes",
        "apt list --installed",
    ]),
    VendorData(Brew, [
        "brew",
        "",
        "brew install $args",
        "brew uninstall $args",
        "brew upgrade $args",
        "brew search $args",
        "brew info $args",
        "brew update",
        "brew upgrade",
        "brew list",
    ]),
    VendorData(Cards, [
        "cards",
        "",
        "cards install $args",
        "cards remove $args",
        "cards install --upgrade $args",
        "cards search $args",
        "cards info $args",
        "cards sync",
        "cards upgrade",
        "cards list",
    ]),
    VendorData(Choco, [
        "choco",
        "--yes",
        "choco install $yes $args",
        "choco uninstall $yes $args",
        "choco upgrade $yes $args",
        "choco search $args",
        "choco info $args",
        "",
        "choco upgrade all $yes",
        "choco list",
    ]),
    VendorData(Dnf, [
        "dnf",
        "--assumeyes",
        "dnf install $yes $args",
        "dnf remove $yes $args",
        "dnf upgrade $yes $args",
        "dnf search $args",
        "dnf info $args",
        "dnf check-update $yes",
        "dnf update $yes",
        "dnf list --installed",
    ]),
    VendorData(Emerge, [
        "emerge",
        "",
        "emerge $args",
        "emerge --depclean $args",
        "emerge --update $args",
        "emerge --search $args",
        "emerge --info $args",
        "emerge --sync",
        "emerge -vuDN @world",
        "qlist -Iv",
    ]),
    VendorData(Eopkg, [
        "eopkg",
        "--yes-all",
        "eopkg install $yes $args",
        "eopkg remove $yes $args",
        "eopkg upgrade $yes $args",
        "eopkg search $args",
        "eopkg info $args",
        "eopkg update-repo $yes",
        "eopkg upgrade $yes",
        "eopkg list-installed",
    ]),
    VendorData(Flatpak, [
        "flatpak",
        "--assumeyes",
        "flatpak install $yes $args",
        "flatpak uninstall $yes $args",
        "flatpak update $yes $args",
        "flatpak search $args",
        "flatpak info $args",
        "",
        "flatpak update $yes",
        "flatpak list",
    ]),
    VendorData(Guix, [
        "guix",
        "",
        "guix install $yes $args",
        "guix remove $yes $args",
        "guix upgrade $yes $args",
        "guix search $args",
        "guix show $args",
        "guix refresh $yes",
        "guix upgrade $yes",
        "guix package --list-installed",
    ]),
    VendorData(NixEnv, [
        "nix-env",
        "",
        "nix-env --install $args",
        "nix-env --uninstall $args",
        "nix-env --upgrade $args",
        "nix-env -qaP $args",
        "nix-env -qa --description $args",
        "nix-channel --update",
        "nix-env --upgrade",
        "nix-env --query --installed",
    ]),
    VendorData(Opkg, [
        "opkg",
        "",
        "opkg install $args",
        "opkg remove $args",
        "opkg upgrade $args",
        "opkg find $args",
        "opkg info $args",
        "opkg update",
        "opkg upgrade",
        "opkg list-installed",
    ]),
    VendorData(Pacman, [
        "pacman",
        "--noconfirm",
        "pacman -S $yes $args",
        "pacman -Rs $yes $args",
        "pacman -S $yes $args",
        "pacman -Ss $args",
        "pacman -Si $args",
        "pacman -Sy $yes",
        "pacman -Syu $yes",
        "pacman -Q",
    ]),
    VendorData(Pkg, [
        "pkg",
        "--yes",
        "pkg install $yes $args",
        "pkg remove $yes $args",
        "pkg install $yes $args",
        "pkg search $args",
        "pkg info $args",
        "pkg update $yes",
        "pkg upgrade $yes",
        "pkg info --all",
    ]),
    VendorData(Pkgman, [
        "pkgman",
        "-y",
        "pkgman install $yes $args",
        "pkgman uninstall $yes $args",
        "pkgman update $yes $args",
        "pkgman search $args",
        "",
        "pkgman refresh $yes",
        "pkgman update $yes",
        "pkgman search --installed-only --all",
    ]),
    VendorData(Ports, [
        "prt-get",
        "",
        "prt-get install $args",
        "prt-get remove $args",
        "prt-get update $args",
        "prt-get search $args",
        "prt-get info $args",
        "ports -u",
        "prt-get sysup",
        "prt-get listinst",
    ]),
    VendorData(Scoop, [
        "scoop",
        "",
        "scoop install $args",
        "scoop uninstall $args",
        "scoop update $args",
        "scoop search $args",
        "scoop info $args",
        "scoop update",
        "scoop update *",
        "scoop list",
    ]),
    VendorData(Slackpkg, [
        "slackpkg",
        "",
        "slackpkg install $args",
        "slackpkg remove $args",
        "slackpkg upgrade $args",
        "slackpkg search $args",
        "slackpkg info $args",
        "slackpkg update",
        "slackpkg upgrade-all",
        "ls -1 /var/log/packages",
    ]),
    VendorData(Snap, [
        "snap",
        "",
        "snap install --classic $args",
        "snap remove $args",
        "snap refresh $args",
        "snap find $args",
        "snap info $args",
        "",
        "snap refresh",
        "snap list",
    ]),
    VendorData(Termux, [
        "termux",
        "--yes",
        "pkg install $yes $args",
        "pkg uninstall $yes $args",
        "pkg install $yes $args",
        "pkg search $args",
        "pkg show $args",
        "pkg update $yes",
        "pkg upgrade $yes",
        "pkg list-installed",
    ]),
    VendorData(Urpm, [
        "urpm",
        "",
        "urpmi $args",
        "urpme $args",
        "urpmi $args",
        "urpmq --fuzzy $args",
        "urpmq -i $args",
        "urpmi.update -a",
        "urpmi --auto-update",
        "rpm --query --all",
    ]),
    VendorData(Winget, [
        "winget",
        "",
        "winget install $args",
        "winget uninstall $args",
        "winget upgrade $args",
        "winget search $args",
        "winget show $args",
        "",
        "winget upgrade --all",
        "winget list",
    ]),
    VendorData(Xbps, [
        "xbps",
        "--yes",
        "xbps-install $yes $args",
        "xbps-remove $yes $args",
        "xbps-install --update $yes $args",
        "xbps-query -Rs $args",
        "xbps-query -RS $args",
        "xbps-install --sync $yes",
        "xbps-install --update $yes",
        "xbps-query --list-pkgs",
    ]),
    VendorData(Yay, [
        "yay",
        "--noconfirm",
        "yay --topdown --cleanafter -S $yes $args",
        "pacman -Rs $yes $",
        "yay --topdown --cleanafter -S $yes $args",
        "yay --topdown -Ss $args",
        "yay --topdown -Si $args",
        "yay --topdown -Sy $yes",
        "yay --topdown -Syu $yes",
        "pacman -Q",
    ]),
    VendorData(Yum, [
        "yum",
        "--assumeyes",
        "yum install $yes $args",
        "yum remove $yes $args",
        "yum update $yes $args",
        "yum search $args",
        "yum info $args",
        "yum check-update $yes",
        "yum update $yes",
        "yum list --installed",
    ]),
    VendorData(Zypper, [
        "zypper",
        "--no-confirm",
        "zypper install $yes $args",
        "zypper remove $yes $args",
        "zypper update $yes $args",
        "zypper search $args",
        "zypper info $args",
        "zypper refresh $yes",
        "zypper update $yes",
        "zypper search --installed-only",
    ]),
];

impl From<Vendor> for VendorData {
    fn from(value: Vendor) -> Self {
        for vendor in VENDORS.iter() {
            if vendor.0 == value {
                return *vendor;
            }
        }
        panic!("unreachable code reached for vendor {:?}", value);
    }
}
