Name:           apmpkg
Version:        1.5.0
Release:        1%{?dist}
Summary:        Package Manager

License:        Apache-2
URL:            https://github.com/kedap/apmpkg
Source0:        %{version}.tar.gz

BuildRequires:  cargo, pkg-config, openssl-devel
Requires:       git, python-pip, wget, fakeroot, rubygem-bundler, rsync, npm

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


%files
%license LICENSE
/usr/share/man/man1/*
/usr/bin/apmpkg
/etc/apmpkg/*


%changelog
* Sat Jul 31 2021 kedap <kedap.dev@protonmail.com>
- Slapt-get for Slackpkg and Npm fixed
