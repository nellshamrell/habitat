$pkg_name = "hab"
$pkg_origin = "core"
$pkg_version = "$(Get-Content $PLAN_CONTEXT/../../VERSION)"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_source = "https://s3-us-west-2.amazonaws.com/habitat-win-deps/hab-win-deps.zip"
$pkg_shasum="1a42a5597843303eac8c31fcafd809dd618a92b69eaea8a51f577c71c7b2fd00"
$pkg_bin_dirs = @("bin")
$pkg_build_deps = @("core/rust")

function Invoke-Prepare {
    $env:PLAN_VERSION               = "$pkg_version/$pkg_release"
    $env:CARGO_TARGET_DIR           = "$HAB_CACHE_SRC_PATH/$pkg_dirname"
    $env:LIB                        = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    = "$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:SODIUM_LIB_DIR             = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:LIBARCHIVE_INCLUDE_DIR     = "$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:LIBARCHIVE_LIB_DIR         = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:OPENSSL_LIBS               = 'ssleay32:libeay32'
    $env:OPENSSL_LIB_DIR            = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:OPENSSL_INCLUDE_DIR        = "$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
}

function Invoke-Unpack {
  Expand-Archive -Path "$HAB_CACHE_SRC_PATH/hab-win-deps.zip" -DestinationPath "$HAB_CACHE_SRC_PATH/$pkg_dirname"
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
    try {
        cargo build --release
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    }
    finally { Pop-Location }
}

function Invoke-Install {
    Copy-Item "$env:CARGO_TARGET_DIR/release/hab.exe" "$pkg_prefix/bin/hab.exe"
    Copy-Item "$HAB_CACHE_SRC_PATH/$pkg_dirname/bin/*" "$pkg_prefix/bin"
}
