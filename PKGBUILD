# Maintainer: kedap <dxhqezk@hi2.in>

pkgname=apmpkg
pkgver=1.0
pkgrel=1
pkgdesc="Un administrador de paquetes universal para linux como modelo: PKGBUILD"
arch=('x86_64')
url="https://github.com/Kedap/apmpkg"
license=('Apache')
depends=('git' 'python-pip' 'python2-pip' 'curl' 'fakeroot' 'ruby-bundler')
makedepends=('cargo')
conflicts=('apmpkg-git')
source=("https://github.com/Kedap/${pkgname}/archive/refs/tags/${pkgver}.tar.gz")
sha256sums=('c31a22ba383026b4b1e6c6f639bb6dca53ad007e6a85bf80df4cb5492af07fe1')

build() {
	cd "$pkgname-$pkgver"
	cargo build --release --locked
}

check() {
	cd "$pkgname-$pkgver"
	cargo build --release --locked
}

package() {
	cd "$pkgname-$pkgver"
	install -Dm 755 "target/release/${pkgname}" -t "${pkgdir}/usr/bin"
	mkdir -p ${pkgname}/etc/apmpkg/iiabc
	cp -r src/iiabc/ ${pkgname}/etc/apmpkg/iiabc
	mkdir -p ${pkgname}/etc/apmpkg/paquetes
	cp "${pkgname}.1" ${pkgname}/usr/share/man/man1
}
