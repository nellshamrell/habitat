pkg_name=hab-builder-api
pkg_origin=core
pkg_version=undefined
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('Apache-2.0')
pkg_source=nosuchfile.tar.gz
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/openssl core/coreutils core/gcc-libs core/zeromq core/libsodium
  core/libarchive)
pkg_build_deps=(core/protobuf core/protobuf-rust core/coreutils core/cacerts
  core/rust core/gcc core/git core/pkg-config core/node core/phantomjs)
pkg_exports=(
  [port]=srv.port
)
pkg_expose=(port)
bin="bldr-api"
pkg_svc_run="$bin start -c ${pkg_svc_path}/config.toml"

do_verify() {
  pushd $PLAN_CONTEXT/../../.. > /dev/null
  pkg_version=`git rev-list master --count`
  pkg_dirname="${pkg_name}-${pkg_version}"
  pkg_prefix="$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
  pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
  popd > /dev/null
}

do_prepare() {
  rm -Rdf $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version
  cp -ra $PLAN_CONTEXT/../../builder-web $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version
  rm -Rdf $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version/node_modules

  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--debug"
  build_line "Building artifacts with \`${build_type#--}' mode"

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive)/include
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export PROTOBUF_PREFIX=$(pkg_path_for protobuf)
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium)/lib
  export LIBZMQ_PREFIX=$(pkg_path_for zeromq)
}

do_build() {
  export LIBRARY_PATH=$LIBZMQ_PREFIX/lib
  build_line "Setting LIBRARY_PATH=$LIBRARY_PATH"

  pushd $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version > /dev/null
  export HOME=$HAB_CACHE_SRC_PATH
  npm install
  for b in node_modules/.bin/*; do
    echo $b
    fix_interpreter $(readlink -f -n $b) core/coreutils bin/env
  done
  npm run postinstall
  npm run dist
  popd > /dev/null

  pushd $PLAN_CONTEXT/.. > /dev/null
  cargo build ${build_type#--debug} --target=$rustc_target --verbose
  popd > /dev/null
}

do_install() {
  cp -vR $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version/dist $pkg_prefix/static

  install -v -D $CARGO_TARGET_DIR/$rustc_target/${build_type#--}/$bin \
    $pkg_prefix/bin/$bin
}

do_strip() {
  if [[ "$build_type" != "--debug" ]]; then
    do_default_strip
  fi
}

# Turn the remaining default phases into no-ops
do_download() {
  return 0
}

do_unpack() {
  return 0
}
