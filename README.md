# Hello Anchor

## Table of Contents

- Getting Started
- Development
- Deployment
- License

### Getting Started

1. Solana CLI

2. Anchor CLI

3. Solana Wallet

### Development

Open `lib.rs` and edit the file to your needs. After completing the program, compile it to produce IDL.

```sh
anchor build
```

Once the IDL is generated, get the program id

```sh
solana address -k target/deploy/hello_anchor-keypair.json
```

### Testing

```sh
anchor test
```

### Deployment

```sh
anchor deploy
```

### License

[GNU Affero General Public License v3.0](https://github.com/YosephKS/hello-anchor/blob/main/LICENSE.md)
