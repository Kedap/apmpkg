Name:           apmpkg
Version:        1.5.1
Release:        1%{?dist}
Summary:        Package Manager

License:        Apache-2
URL:            https://github.com/kedap/apmpkg
Source0:        %{version}.tar.gz

BuildRequires:  cargo, pkg-config, openssl-devel
Requires:       git, python-pip, wget, fakeroot, rubygem-bundler, npm

%description
A Package Manager as model: PKGBUILD

%prep
%autosetup


%build
cargo build --release --locked


%install
rm -rf $RPM_BUILD_ROOT
install -Dm 755 "target/release/apmpkg" -t "%{buildroot}/usr/bin"
mkdir -p %{buildroot}/etc/apmpkg/iiabc
cp -r src/iiabc/ %{buildroot}/etc/apmpkg/
mkdir -p %{buildroot}/etc/apmpkg/paquetes
install -Dm 644 "man/apmpkg.1" -t %{buildroot}/usr/share/man/man1
install -Dm 644 "man/apmpkg-en.1" -t %{buildroot}/usr/share/man/man1
install -Dm 644 "completions/apmpkg.bash-completion" -t %{buildroot}/usr/share/bash-completion/bash_completion
install -Dm 644 "completions/_apmpkg" -t %{buildroot}/usr/share/zsh/site-functions
install -Dm 644 "completions/apmpkg.fish" -t %{buildroot}/usr/share/fish/vendor_completions.d


%files
%license LICENSE
/usr/share/man/man1/*
/usr/bin/apmpkg
/etc/apmpkg/*
/usr/share/bash-completion/bash_completion
/usr/share/zsh/site-functions
/usr/share/fish/vendor_completions.d


%changelog
* Fri Feb 12 2022 kedap <kedap.dev@protonmail.com>
- Adding Bash, Zsh and Fish completions and fix bugs. see more https://github.com/kedap/apmpkg
