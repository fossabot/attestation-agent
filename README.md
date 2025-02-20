# Attestation Agent
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fconfidential-containers%2Fattestation-agent.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fconfidential-containers%2Fattestation-agent?ref=badge_shield)


Attestation Agent (AA for short) is a user space service for attestation procedure. 
In Kata Confidential Containers (Kata CC for short), AA implements the WrapKey API defined by the key-provider protocol, 
which is responsible for performing attestation and obtaining the Key Encryption Key (KEK for short) from Key Broker Service (KBS for short),
or requesting KBS to decrypt the encrypted payload stored in the image layer annotation.


Current consumers of AA include: 

- [ocicrypt-rs](https://github.com/containers/ocicrypt-rs)
- [ocicrypt](https://github.com/containers/ocicrypt)

## Usage

Here are the steps of building and running AA:

### Build

Build and install AA with all KBC modules:

```shell
git clone https://github.com/containers/attestation-agent
cd attestation-agent
make && make install
```

or explicitly specify the KBS modules it contains. Taking `sample_kbc` as example:

```shell
make KBC=sample_kbc
```

#### Musl 

To build and install with musl, just run:
```shell
make LIBC=musl && make install
```

### Run

For help information, just run:

```shell
attestation-agent --help
```

Start AA and specify the endpoint of AA's gRPC service:

```shell
attestation-agent --keyprovider_sock 127.0.0.1:47777 --getresource_sock 127.0.0.1:48888
```

If you want to see the runtime log:
```
RUST_LOG=attestation_agent attestation-agent --keyprovider_sock 127.0.0.1:47777 --getresource_sock 127.0.0.1:48888
```

## Supported KBC modules

AA provides a flexible KBC module mechanism to support different KBS protocols required to make the communication between KBC and KBS. If the KBC modules currently supported by AA cannot meet your use requirement (e.g, need to use a new KBS protocol), you can write a new KBC module complying with the KBC development [GUIDE](docs/kbc_module_development_guide.md). Welcome to contribute new KBC module to this project!

List of supported KBC modules: 

| KBC module name    | README                                                              | KBS protocol | Maintainer                |
| ------------------ | ------------------------------------------------------------------- | ------------ | ------------------------- |
| sample_kbc         | Null                                                                | Null         | Attestation Agent Authors |
| offline_fs_kbc     | [Offline file system KBC](src/kbc_modules/offline_fs_kbc/README.md) | Null         | IBM                       |
| eaa_kbc            | [EAA KBC](src/kbc_modules/eaa_kbc/README.md)                        | EAA protocol | Alibaba Cloud             |
| offline_sev_kbc    | [Offline SEV KBC](src/kbc_modules/offline_sev_kbc/README.md)        | Null         | IBM                       |


## Tools

- [Sample Keyprovider](./sample_keyprovider): A simple tool for encrypting container images with skopeo, please refer to its [README](./sample_keyprovider/README.md).



## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fconfidential-containers%2Fattestation-agent.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fconfidential-containers%2Fattestation-agent?ref=badge_large)