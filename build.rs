// Copyright (c) 2021 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

fn main() -> shadow_rs::SdResult<()> {
    tonic_build::compile_protos("protos/keyprovider.proto")?;
    tonic_build::compile_protos("protos/getresource.proto")?;

    #[cfg(feature = "eaa_kbc")]
    {
        println!("cargo:rustc-link-search=native=/usr/local/lib/rats-tls");
        println!("cargo:rustc-link-lib=dylib=rats_tls");
    }

    shadow_rs::new()
}
