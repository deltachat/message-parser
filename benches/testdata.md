
HELLO world this is a test document for the benchmarking process, it contains **bold**, *italics* and ~~struck through sections~~.



email@host email@domain.tld mailto:email@exxample.com

https://delta.chat?test=1234&y=4
http://delta.chat?test=1234&y=4

https://delta.chat/hello?test=1234&y=4



Then a text containing a delimited link <https://delta.chat/hello?test=1234&y=4> then a [labeled link](https://delta.chat/hello?test=1234&y=4) and a #hashtag, cause why not.

`inline code` and more useless text: 1+1 != 3 ; what a user may or may not write in a message somehow.


tons of data from awesome bitcoin cash list:

<div align="center">
  <img width="1400px" alt="awesome bitcoin cash" src="./awesome-bitcoin-cash.dark.svg">
</div>
<br/>
<div align="center">
A curated list of Bitcoin Cash projects &amp; resources <br>
<a href="https://awesome.re">
  <img src="https://awesome.re/badge.svg" alt="awesome" style="height:12px;border:10;">
</a>
<link rel="manifest" href="/assets/favicon/site.webmanifest">
<br />
Bitcoin Cash (BCH) is a project to scale bitcoin on-chain as an electronic peer-to-peer payment system for the world. 🚀

</div>
<br/>

📤 [a mobile friendly version](https://awesomebitcoin.cash) of this [project](https://github.com/2qx/awesome-bitcoin-cash) is formatted [from markdown](https://github.com/2qx/awesome-bitcoin-cash/blob/master/README.md) by github pages.

Pull requests are welcome, please see [the contribution guidelines](CONTRIBUTING.md).
<br/>

[![Check Links](https://github.com/2qx/awesome-bitcoin-cash/actions/workflows/links.yml/badge.svg)](https://github.com/2qx/awesome-bitcoin-cash/actions/workflows/links.yml)


# Contents

- [Contents](#contents)
- [Getting Started](#getting-started)
- [State of the Project](#state-of-the-project)
- [Whitepaper](#whitepaper)
- [Open-Source Wallets](#open-source-wallets)
  - [Mobile](#mobile)
  - [Desktop](#desktop)
    - [Electron-Cash Plugins](#electron-cash-plugins)
  - [Cli](#cli)
  - [Browser](#browser)
  - [Paper/Offline Generator](#paperoffline-generator)
- [Podcasts, News, Media](#podcasts-news-media)
- [Projects Built on Bitcoin Cash](#projects-built-on-bitcoin-cash)
  - [Apps (Social)](#apps-social)
  - [Crowdfunding](#crowdfunding)
  - [BCH Native Decentralized Finance](#bch-native-decentralized-finance)
  - [Collectables](#collectables)
  - [Entertainment](#entertainment)
  - [Exchanges](#exchanges)
    - [Centralized](#centralized)
    - [More decentralized](#more-decentralized)
  - [Oracles](#oracles)
  - [Faucets](#faucets)
  - [Network](#network)
    - [Explorers](#explorers)
    - [Testnet Explorers](#testnet-explorers)
  - [Services](#services)
  - [Utilities](#utilities)
  - [Web](#web)
  - [See Also](#see-also)
- [Merchants and Services Accepting Bitcoin Cash](#merchants-and-services-accepting-bitcoin-cash)
  - [A Short List](#a-short-list)
  - [Geographic lists](#geographic-lists)
  - [Projects dedicated to listing or enabling eCommerce.](#projects-dedicated-to-listing-or-enabling-ecommerce)
  - [Some Charities and Foundations](#some-charities-and-foundations)
- [eCommerce Merchant Resources](#ecommerce-merchant-resources)
  - [Bitcoin Cash Open-Source plugins](#bitcoin-cash-open-source-plugins)
  - [Point of Sale Clients](#point-of-sale-clients)
  - [Non-Custodial Payment Processors](#non-custodial-payment-processors)
  - [BCH-to-Fiat Payment Processors](#bch-to-fiat-payment-processors)
  - [Payment Processor Status](#payment-processor-status)
- [Documentation](#documentation)
  - [General](#general)
  - [Base Protocol](#base-protocol)
    - [Secondary protocols](#secondary-protocols)
  - [Discussion](#discussion)
  - [CHIP Process](#chip-process)
    - [Previous consensus changes, May 2023:](#previous-consensus-changes-may-2023)
  - [Bitcoin Script](#bitcoin-script)
- [Software](#software)
  - [Full Nodes](#full-nodes)
    - [Developer Resources](#developer-resources)
  - [Open-Source Teams Building on Bitcoin Cash](#open-source-teams-building-on-bitcoin-cash)
  - [Simple Payment Verification (SPV)](#simple-payment-verification-spv)
  - [Libraries \& SDKs](#libraries--sdks)
    - [Language Agnostic](#language-agnostic)
    - [Typescript](#typescript)
    - [Javascript](#javascript)
    - [Python](#python)
    - [Rust](#rust)
    - [Java](#java)
    - [C](#c)
    - [PHP](#php)
    - [R](#r)
- [Endorsements](#endorsements)
  - [The Adaptive Blocksize Limit Algorithm (ebaa) CHIP for the May 2024 BCH Upgrade is AWESOME!](#the-adaptive-blocksize-limit-algorithm-ebaa-chip-for-the-may-2024-bch-upgrade-is-awesome)
  - [The CashTokens and P2SH32 CHIP Proposals for the May 2023 BCH Upgrade are AWESOME!](#the-cashtokens-and-p2sh32-chip-proposals-for-the-may-2023-bch-upgrade-are-awesome)
- [The Archive](#the-archive)
  - [Bitcoin Script tools](#bitcoin-script-tools)
  - [Simple Ledger Protocol (SLP Token)](#simple-ledger-protocol-slp-token)
    - [Protocols](#protocols)
    - [Libraries](#libraries)
  - [SLP Token Projects](#slp-token-projects)

# Getting Started

- [bitcoincash.org](https://bitcoincash.org) - A general multi-lingual introduction.
- [BCH Info](https://bch.info/) - Multilingual site for general information about bitcoin cash.
- [BCHFAQ.com](https://bchfaq.com/) [[code]](https://github.com/fixthetracking/Bitcoin-Cash-FAQ) - Learn the fundamentals of Bitcoin Cash by getting simple answers to your basic questions.
- [Why Bitcoin Cash?](https://whybitcoincash.com/) [[archive]](https://web.archive.org/web/20230228125654/https://whybitcoincash.com/) - The revolution will not be censored.
- [Bitcoin.com Getting Started](https://www.bitcoin.com/get-started/) - Comprehensive introduction for general audiences.
- [Why Cryptocurrencies?](https://whycryptocurrencies.com/toc.html) [[code]](https://github.com/treeman/why_cryptocurrencies) - An explanation on why cryptocurrencies were created, what they do differently and why they matter.

# State of the Project

- [Three Years In: A Bitcoin Cash Update From One of Its Founders](https://news.bitcoin.com/three-years-in-a-bitcoin-cash-update-from-one-of-its-founders/) - by Jonald Fyookball

# Whitepaper

"Bitcoin: A Peer-to-Peer Electronic Cash System" by Satoshi Nakamoto.

Bitcoin Cash is one chain of Satoshi Nakamoto's blockchain invention which was deliberately hard-forked on August 1st, 2017. It shares the whitepaper, first block, and all bitcoin block history prior to the fork. It attempts to implement the central idea outlined in that paper.

Below is a copy of the original nine page whitepaper:

- [Archived copy](https://web.archive.org/web/20100704213649if_/http://www.bitcoin.org:80/bitcoin.pdf) of the bitcoin whitepaper from bitcoin.org.
- [bitcoin whitepaper](https://gateway.ipfs.io/ipfs/QmRA3NWM82ZGynMbYzAgYTSXCVM14Wx1RZ8fKP42G6gjgj) via ipfs.
- Websites hosting the bitcoin whitepaper [[wayback archive]](http://web.archive.org/web/20210516141704if_/https://blockchair.com/bitcoin/whitepaper), with sha256 hashes calculated as of May 16th 2021.
- [As a webcomic](https://web.archive.org/web/20230215013643/https://whitepaper.coinspice.io/) [[中文]](https://web.archive.org/web/20230315051200/https://whitepaper.coinspice.io/cn) [[日本語]](https://web.archive.org/web/20200217125719/https://www.bitcoin.jp/what-is-bitcoin/bitcoin-whitepaper-comic/) - Bitcoin Whitepaper web comic by Scott McCloud.
- [Instructions and code](https://bitcoin.stackexchange.com/questions/35959/how-is-the-whitepaper-decoded-from-the-blockchain-tx-with-1000x-m-of-n-multisi) for building the original paper encoded on the blockchain on 2013-04-06.

# Open-Source Wallets

Below are non-custodial open-source wallets that use features specific to Bitcoin Cash.

**[Best BCH Wallets](https://www.bestbchwallets.com)** is a tool for selecting a wallet based on operating system and features.

## Mobile

- 🔵 [Electron-Cash](https://electroncash.org) - Android [[code]](https://github.com/Electron-Cash/Electron-Cash/tree/master/android) and iOS [[code]](https://github.com/Electron-Cash/Electron-Cash/tree/master/ios) versions available with more limited functionality.
- 🔵 [Paytaca](https://www.paytaca.com/) [[apk]](https://github.com/paytaca/paytaca-app/releases) [[code]](https://github.com/paytaca/paytaca-app) - A mobile wallet for Android, iOS and ChromeOS
- [Flowee Pay](https://flowee.org/products/pay/) [[code]](https://codeberg.org/Flowee/pay/) [[apk]](https://flowee.org/products/pay/) [[docs]](https://codeberg.org/Flowee/Pay/wiki) - A user friendly wallet for Android and Linux desktop.
- [Selene Wallet](https://selene.cash/) [[code]](https://git.xulu.tech/selene.cash/selene-wallet/) - Easy, no-hassle, instant payments in the palm of your hand.
- [Stack Wallet](https://stackwallet.com/) [[code]](https://github.com/cypherstack/stack_wallet) - Multicoin wallet with UTXO (coin) control.
- [Cake Wallet](https://cakewallet.com/) [[code]](https://github.com/cake-tech/cake_wallet) [[apk]](https://github.com/cake-tech/cake_wallet/releases) - An open source wallet for iOS and Android supporting XMR and other currencies.
- 🔵 [zapit](https://zapit.io/#/)* - A native, non-custodial Bitcoin Cash wallet for iOS and Android. *Not open source

## Desktop
- 🔵 [Electron Cash CashToken](https://electroncash.org) [[release]](https://github.com/Electron-Cash/Electron-Cash/releases/tag/4.3.0) [[code]](https://github.com/Electron-Cash/Electron-Cash/) - Electron Cash with CashTokens.
- [Flowee Pay](https://flowee.org/products/pay/) [[code]](https://codeberg.org/flowee/pay) - A payment solution, a wallet, a basis for your new product. But currently just a desktop wallet.
- 🔵 [Cashonize (quasar)](https://github.com/cashonize/cashonize-quasar/releases/tag/v0.0.2) [[code]](https://github.com/cashonize/cashonize-quasar) - Cashonize rewrite with Quasar & Vue-js

### Electron-Cash Plugins

- [Flipstarter Plugin](https://gitlab.com/flipstarter/flipstarter-electron-cash) - plugin for crowdfunding.
- [Nostron](https://github.com/Electron-Cash/Nostron/) - Nostron is a plugin for the Electron-Cash BCH wallet.
- [Inter-Wallet Transfer plugin](https://github.com/KarolTrzeszczkowski/Inter-Wallet-Transfer-EC-plugin) - A plugin, that sends your coins to another wallet one by one, every time to a fresh address. 
- [Mecenas Plugin](https://github.com/KarolTrzeszczkowski/Mecenas-recurring-payment-EC-plugin/releases) - recurring payments.
- [Last Will](https://github.com/KarolTrzeszczkowski/Electron-Cash-Last-Will-Plugin) - dead man smart contract creation.
- [HODL](https://github.com/mainnet-pat/hodl_ec_plugin/) - smart contract plugin for Electron Cash to timelock funds.
- [AutoCove](https://github.com/TinosNitso/AutoCove-Plugin) - Electrum-cash script decoder.

## Cli

- [bitcore-wallet](https://github.com/bitpay/bitcore/tree/master/packages/bitcore-wallet) - A command line wallet used for BitPay wallets.

## Browser
- 🔵 [Cashonize](https://cashonize.com/) [[code]](https://github.com/cashonize/wallet) -  An experimental web wallet for CashTokens.
- [PSF wallet](https://wallet.fullstack.cash/) [[code]](https://github.com/Permissionless-Software-Foundation/gatsby-ipfs-web-wallet) - An web wallet with SLP support.
- 🔵 [Microfi Wallet](https://microfi.eu/wallet/) - Microfi Free Flow Wallet
- [BCH Merchant PoS](https://pos.cash) [[code]](https://github.com/softwareverde/pos-cash) - Bitcoin Cash Web Point of Sale, from SoftwareVerde.

## Paper/Offline Generator

- [Cash Address Generator](https://cashaddress.org/) [[code]](https://github.com/theantnest/bccaddress) - reputable javascript address generator suitable for offline use.
- [Bitcoin.com Paper Wallet](https://paperwallet.bitcoin.com/) [[code]](https://github.com/Bitcoin-com/paperwallet.bitcoin.com) - A fork of the cashaddress.org paper wallet
- Keep Bitcoin Free Paper Wallet [[code]](https://github.com/KeepBitcoinFree-org/paper.keepbitcoinfree.org) - A fork of the Bitcoin.com paper wallet
- [BCH Gifts](https://gifts.bitcoin.com/) - generate reclaimable preloaded paper private keys as gifts.

# Podcasts, News, Media

Bitcoin Cash focussed media and content.

- [The Bitcoin Cash Podcast](https://www.bitcoincashpodcast.com) - Available on [Youtube](https://www.youtube.com/channel/UCsrDsJnHFnkMnJhEslofyPQ) and [RSS](https://rss.com/podcasts/bitcoincashpodcast/) audio versions, plus other video and podcast platforms (see links at bottom of website).
- [Bitcoin Cash Foundation](https://bitcoincashfoundation.org/) Weekly News - Available on [Youtube](https://www.youtube.com/@BitcoinCashFoundation) and [Telegram](https://t.me/BCHFNews) 
- General Protocol Spaces -  Available on [Youtube](https://www.youtube.com/watch?v=707-DPzhdA8&list=PLcIK2erO9hWyM56FYkUAilfUmABbwpu7U) and twitter.


# Projects Built on Bitcoin Cash

All of these apps are mostly stable and active. Always check the notes of a particular project before risking a large sum of value. Links are checked on a weekly basis, but function is not checked.

## Apps (Social)

- [read.cash](https://read.cash) - a conventionally hosted long-format blogging platform, with BCH tipping for content.
- [memo.cash](https://memo.cash) - short message social media site with decentralized SLP token exchange.
- [Cashrain](https://cashrain.com/) - A platform where creators create communities for their members.
- [noise.app](https://noise.app) - An invite only Bitcoin Cash powered micro-blogging platform.
- [OnlyCoins](https://onlycoins.com/) - Adult content monetization platform.
- [Glimpse.cash](https://glimpse.cash/) - A pay per view video hosting and streaming platform.
- [Gaze.cash](https://gaze.cash/) - A more lenient pay-per-view video platform.
- [WhoTipped.it](https://whotipped.it/) - Last tips given on memo.cash
  
## Crowdfunding

- [flipstarter](https://flipstarter.cash/) [[Introduction]](https://read.cash/@flipstarter/introducing-flipstarter-695d4d50) [[code]](https://gitlab.com/flipstarter/backend) - a crowd funding app using anyone can pay multisig transactions.
- IPFS Flipstarter [[code]](https://gitlab.com/ipfs-flipstarter) - An IPFS flipstarter campaign site.

## BCH Native Decentralized Finance

[DefiLlama](https://defillama.com/chain/Bitcoincash) - Statistics for Bitcoin Cash Defi.

- [BCH Bull](https://bchbull.com/) [[app]](https://app.bchbull.com/) - Permissionless leverage and hedging using the Anyhedge protocol.
- 🔵 [TapSwap](https://tapswap.cash/) - An open marketplace for fungible and non-fungible tokens.
- 🔵 [Cauldron](https://www.cauldron.quest/) [[whitepaper]](https://www.cauldron.quest/_files/ugd/ae85be_b1dc04d2b6b94ab5a200e3d8cd197aa3.pdf) - A Constant product market maker contract
- [Unspent](https://unspent.cash) [[code]](https://github.com/2qx/unspent) [[cli]](https://www.npmjs.com/package/unspent) [[docs]](https://unspent.app/documentation) -  An irrevocable perpetuity app
- 🔵 [Emerald DAO](https://emerald-dao.cash/) [[app]](https://emerald-dao.vercel.app/) [[code]](https://gitlab.com/0353F40E/emerald-dao/) - A simple Bitcoin Cash DAO template which acts as a fixed-term deposit savings vault.
- 🔵 [Wrapped Cash](https://wrapped.cash/) [[code]](https://gitlab.com/dagurval/wrapped-cash) - Bitcoin Cash wrapped as a CashToken



## Collectables 

- 🔵 [BCH Guru NFTs](https://nfts.bch.guru) - a premier collection of NFTs
- 🔵 [Ghostwriter](https://ghostwriter.pages.dev/) - Text based NFT minting
- 🔵 [Bitcats Heroes](https://bitcatsheroes.club/) - Collectibele NFT series with non-custodial minting contract.
- 🔵 [CashNinjas](https://ninjas.cash/) [[code]](https://github.com/cashninjas) - an NFT collection leveraging the new CashTokens technology.
  
  
## Entertainment

- [bch.games](https://bch.games/) - dice and numbers game.
- 🔵 [BCU Guru](https://bch.guru) - A peer to peer price prediction game on Bitcoin Cash
- 🔵 [DogeCash](https://dogecash.uwu.ai/) - Don't let your dreams be memes
- [craft.cash](https://craft.cash/) [[code]](https://github.com/blockparty-sh/craft.cash) - Voxel world stored on Bitcoin Cash.
- [Satoshi dice](https://www.satoshidice.com/) - a provably fair dice game.
- [Spin BCH](https://SpinBCH.com) - Spinning wheel based gambling using zero-conf

## Exchanges

Bitcoin Cash is supported on hundreds of exchanges, these are a few.

### Centralized

- [CoinEx](https://www.coinex.com/) - A BCH friendly exchange with automatic coin-splitting

### More decentralized

- [Thorchain Swap](https://app.thorswap.finance/) - Swap native assets directly with any non-custodial wallet across nine blockchains.
- [Komodo Wallet](https://app.komodoplatform.com/) - Decentralized exchange with desktop clients supporting BCH and many UTXO coins, ETH, ERC-20 tokens

## Oracles

- [Oracles.Cash](https://oracles.cash/) [[Best Practices]](https://gitlab.com/GeneralProtocols/priceoracle/library#best-practices-for-price-oracle-consumers) [[spec]](https://gitlab.com/GeneralProtocols/priceoracle/specification) - Price oracles for Bitcoin Cash

## Faucets

- 🔵 [Testnet Faucet](https://tbch.googol.cash/) [[code]](https://gitlab.com/uak/light-crypto-faucet)
- 🔵 [`unspent`](https://www.npmjs.com/package/unspent?activeTab=readme) [[code]](https://github.com/2qx/unspent) - an javascript package with commands for faucets.
- BCH Testnet Faucet [[code]](https://github.com/christroutner/testnet-faucet2/) - Fullstack.cash faucet for tBCH.

## Network

- [fork.lol](https://fork.lol) - Site to monitor network health in relation to BTC.
- [Johoe's Bitcoin Mempool Statistics](https://jochen-hoenicke.de/queue/) [[code]](https://github.com/jhoenicke/mempool) - Colorful mempool graphs.
- [Electrum Server Status for BCH](https://1209k.com/bitcoin-eye/ele.php?chain=bch) [[or tBCH]](https://1209k.com/bitcoin-eye/ele.php?chain=tbch) - A 1209k hosted list of electrum servers
- [Tx Street](https://txcity.io/v/bch-eth) [[code]](https://github.com/txstreet/txstreet) - a live blockchain transaction and mempool visualizer.
- [Bitcoin Energy Statistics](https://www.monsterbitar.se/~jonathan/energy/) - A comparison of energy usage for BCH and BTC.

### Explorers
- 🔵 [Blockchain Explorer](https://explorer.bch.ninja/) [[code]](https://github.com/sickpig/bch-rpc-explorer) [[mirror: BU]](https://explorer.bitcoinunlimited.info/) [[mirror: electroncash.de]](https://explorer.electroncash.de) - Database-free, self-hosted Bitcoin Cash explorer, via RPC.
- 🔵 [Bitcoin Cash Explorer](https://explorer.salemkode.com/) [[code]](https://github.com/salemkode/explorer) - A Bitcoin Cash Explorer with CashTokens, by SalemKode.
- 🔵 [3xpl.com BCH Explorer](https://3xpl.com/bitcoin-cash) [[code]](https://github.com/3xplcom)- Fastest ad-free universal block explorer.
- [BCH Explorer](https://explorer.melroy.org/) [[code]](https://gitlab.melroy.org/bitcoincash/explorer) - Bitcoin Cash Explorer by Melroy van den Berg
- [Blockchair BCH Explorer](https://blockchair.com/bitcoin-cash) - Universal blockchain explorer and search engine.
- [Blockchain.com BCH explorer](https://www.blockchain.com/explorer?view=bch) - Established blockchain explorer.
- 🔵 [BCH CashTokens NFT Viewer](https://viewer.sploit.cash) [[code]](https://github.com/acidsploit/cashtokens-nft-viewer) -  Sploit's NFT viewer.
  ### Testnet Explorers
  - 🔵 [Chipnet (im_uname)](https://chipnet.imaginary.cash)
  - 🔵 [Chipnet (chaingraph)](https://chipnet.chaingraph.cash)
  - 🔵 [Chipnet (bch.ninja)](https://chipnet.bch.ninja)
  - [Testnet [old]](https://texplorer.bitcoinunlimited.info/), [[mirror]](https://testnet-explorer.electroncash.de/)
- [Chaingraph](https://chaingraph.cash/) [[code]](https://github.com/bitauth/chaingraph) - A multi-node blockchain indexer and GraphQL API.
- [CoinGecko API](https://www.coingecko.com/api/documentation) - Free tier api for price data.
- [Blockchair Bulk Data](https://gz.blockchair.com/bitcoin-cash/) - Daily compressed dumps of blockchain data.
- [CashFusion Stats](https://fusionstats.redteam.cash/) - Data on privacy-enhancing CashFusion transactions.
- [Mempool Project](https://bchmempool.cash/) - A Bitcoin Cash (BCH) adaptation of the mempool open-source explorer.
- [bitcoinfees.cash](https://bitcoinfees.cash/) - bitcoin chain fee juxtaposition.

## Services

- 🔵 [OpenTokenRegistry](https://otr.cash/) [[code]](https://github.com/OpenTokenRegistry/otr.cash) - Community-Verified Token Information
- 🔵 [IPFS-BCH](https://ipfs-bch.pat.mn/) [[code]](https://github.com/mainnet-pat/ipfs-bch.pat.mn) - IPFS file pinning service with on-chain settlement
- [CashTags](https://tags.infra.cash/) [[code]](https://github.com/developers-cash/cashtags-server) - Service for printable QR Codes (Payment URLs) whose value amounts can be specified in fiat (e.g. USD).
- [SideShift.ai](https://sideshift.ai/) - enables HUMANS and AI to shift between 30+ cryptocurrencies.
- 🔵 [Token Stork](https://tokenstork.com/) - A CashToken market capitalization explorer.
- 🔵 [Token Explorer](https://tokenexplorer.cash/) - A Token explorer for CashTokens.
- [Chaintip Bounties](https://github.com/chaintip/bounties/blob/master/README.md#available-bounties) - BCH bot for github bounties.
- [BCH.gg](https://bch.gg/) - Bitcoin Cash URL Shortener

## Utilities

- [CashAccount](https://www.cashaccount.info/) - Online utility for cashaccounts (address handles).
- 🔵 [Bitauth IDE](https://ide.bitauth.com/) [[code]](https://github.com/bitauth/bitauth-ide) [[walk-thru]](https://www.youtube.com/watch?v=o-igo-adS8E) - An online IDE for developing Bitcoin Cash contracts.
- 🔵 [CashTokens Studio](https://cashtokens.studio/) -  CashToken and Authkey creation tool ([chipnet](https://chipnet.cashtokens.studio/))
- [Bitcoin.com Tools](https://tools.bitcoin.com/) - A mix of Bitcoin utilities.
- 🔵 [CashTokens Airdrop Tool](https://github.com/mr-zwets/airdrop-tool) - A command line utility to airdrop fungible tokens to NFT holders.
  
## Web

- [Bitcoin Paywall](https://wordpress.org/plugins/bitcoin-paywall/) [[code]](https://plugins.trac.wordpress.org/browser/bitcoin-paywall/) - Wordpress paywall plugin

## See Also

These are other projects dedicated to listing projects in the Bitcoin Cash ecosystem:

- [HelpMe Cash](https://helpme.cash/) - A collection of links to things related to the cryptocurrency Bitcoin Cash
- [Bitcoin Cash Projects](https://www.bitcoin.com/bitcoin-cash-projects/) - maintained by bitcoin.com.
- [BCH Developments](https://keepbitcoinfree.org/bch-dev/) - list maintained by KeepBitcoinFree.
- [Canonical awesome-bitcoin-cash](https://github.com/dsmurrell/awesome-bitcoin-cash) - the original.
- [Mainnet Cash List](https://mainnet.cash/projects.html) - A list of projects maintained at mainnet.cash
- [BCHGANG Link Directory](https://bchgang.org) - A directory of links about the cryptocurrency Bitcoin Cash: wallets, merchants, exchanges, tools, references, block explorer, developer guides, tutorials and more.

# Merchants and Services Accepting Bitcoin Cash

## A Short List

These vendors have accepted bitcoin for years and are committed (or sympathetic) toward the idea of electronic cash payments.

Although some of these may appear to only accept Bitcoin (BTC), they do, in fact, accept Bitcoin Cash also.

- [Namecheap](https://namecheap.com) - dns, ssl and some packaged hosting.
- [keys4coins](https://www.keys4coins.com/) - Buy PC games and gift cards with cryptocurrency.
- [alfa.top](https://alfa.top/) - Buy mobile top-up (credit) and internet with cryptocurrency.
- [CheapAir](https://www.cheapair.com) - for your travel needs.
- [Travala](https://www.travala.com) - for your travel needs.
- [items sold by Newegg](https://kb.newegg.com/knowledge-base/using-bitcoin-on-newegg/) - good for a great headset.

## Geographic lists

- [OpenStreetMap BCH Tag](https://overpass-turbo.eu/?w=%22currency%3ABCH%22%3D%22yes%22+global&R) - Entries tagged with `currency:BCH=yes` in OSM.
- [Bitcoin.com map](https://map.bitcoin.com/) - website and mobile app for discovering merchants, formerly marco coino.
- [Bmap.app](https://bmap.app/) - ₿itcoin places all around the world!
- [where2cash](https://where2.cash/) - Bitcoin Cash Map using OpenStreeMap data.
- [map.usecash](https://map.usecash.com)[[code]](https://github.com/modenero/use-cash) - Use Cash map built by Modenero.

## Projects dedicated to listing or enabling eCommerce.

- [Use.Cash](https://usecash.com/) - Guide for using cryptocurrency like cash.
- [Bitgree](https://www.bitgree.com) - service to privately purchase goods on Amazon.com and others at a discount.

## Some Charities and Foundations

Just some good charities for the world at large.

- [Tails](https://tails.boum.org/donate/index.en.html) - The Amnesic Incognito Live System, is a security-focused Debian-based Linux distribution aimed at preserving privacy and anonymity.
- [Save the Children](https://files.savethechildren.org/cryptocurrency-donation/) - **A United Kingdom based charity, founded in 1919**, to improve the lives of children through better education, health care, and economic opportunities, as well as providing emergency aid in natural disasters, war, and other conflicts. (Cryptocurrency donations are powered by [The Giving Block](https://www.thegivingblock.com/))
- [The Internet Archive](https://blockchair.com/bitcoin-cash/address/1Archive1n2C579dMsAu3iC6tWzuQJz8dN) - 1Archive1n2C579dMsAu3iC6tWzuQJz8dN
- [Bitpay Charity Directory](https://bitpay.com/directory/nonprofits) A list of charities that accept Bitcoin Cash and other cryptocurrencies.

# eCommerce Merchant Resources

## Bitcoin Cash Open-Source plugins

- [CryptoWoo for WooCommerce](https://github.com/WeProgramIT/cryptowoo-bitcoin-cash-addon) - Bitcoin Cash integration for CryptoWoo

## Point of Sale Clients

- 🔵 [Paytaca](https://www.paytaca.com/) [[apk]](https://github.com/paytaca/paytaca-app/releases) [[code]](https://github.com/paytaca/paytaca-app) - A mobile wallet with integrated POS.
- [pos.cash](https://pos.cash) [[code]](https://github.com/softwareverde/pos-cash) - a non-custodial web-based point of sale BCH client.
  
## Non-Custodial Payment Processors

- [Prompt.cash](https://prompt.cash) [[demo]](https://www.youtube.com/watch?v=8TIpZW1P_9M) [[docs]](https://prompt.cash/pub/docs/#introduction) - a non-custodial Bitcoin Cash payment gateway
- [Cash Pay Server](https://github.com/developers-cash/cash-pay-server-js) [[docs]](https://developers-cash.github.io/cash-pay-server-js/) - a self-hostable NodeJS micro-service that can be used to handle BIP70 and JSON Payment Protocol invoices for Bitcoin Cash (BCH)

## BCH-to-Fiat Payment Processors

- [BitPay developer Integrations](https://bitpay.com/integrations/) [[api docs]](https://bitpay.com/docs)

## Payment Processor Status

- [status.bitpay.com](https://status.bitpay.com/) - Current status with recent incidents.

# Documentation

## General

- [developers.cash](https://developers.cash/) - many useful resources
- [Permissionless Software Foundation Videos](https://psfoundation.cash/video/)
- [Electron Cash Wiki](https://wiki.electroncash.de/wiki/Main_Page)

## Base Protocol

- [BCH Specification](https://flowee.org/docs/spec/) - Specification hosted by flowee.org.
- [Bitcoin Cash Protocol Documentation](https://documentation.cash/) [[code]](https://github.com/SoftwareVerde/bitcoin-cash-specification) - maintained by Software Verde.
- [reference.cash](https://reference.cash) - protocol documentation
- [Upgrade specs](https://upgradespecs.bitcoincashnode.org/) - Bitcoin Cash upgrade specifications as implemented by BCHN.

### Secondary protocols

[Bitcoin Cash Standards](https://bitcoincashstandards.org) is a site dedicated to collecting, some of which are listed below:

- [AnyHedge](https://anyhedge.com/) [[docs]](https://anyhedge.com/developers/) [[code]](https://gitlab.com/GeneralProtocols/anyhedge) - Decentralized hedge solution against arbitrary commodities for Bitcoin Cash
- 🔵 [Bitcoin Cash Metadata Registries (BCMR)](https://cashtokens.org/docs/bcmr/chip/) [[code]](https://github.com/bitjson/chip-bcmr) - A standard for sharing authenticated metadata between Bitcoin Cash wallets.
- [Cashaddr](https://upgradespecs.bitcoincashnode.org/cashaddr/) - Format for Bitcoin Cash addresses.
- [Cash Accounts](https://gitlab.com/cash-accounts/specification/blob/master/SPECIFICATION.md) - attach a human readable name to Bitcoin Cash addresses.
- CashFusion(https://cashfusion.org) [[spec]](https://github.com/cashshuffle/spec/blob/master/CASHFUSION.md) - a privacy protocol for privately and trustlessly joining coin amounts.
- [CashID](https://gitlab.com/cashid/protocol-specification) - Specification using Bitcoin Cash for secure authentication.
- 🔵 [CashTokens](https://cashtokens.org/) [[code]](https://github.com/cashtokens/cashtokens.org) - Specification for CashTokens.
- [Electrum Cash Protocol (Fulcrum)](https://electrum-cash-protocol.readthedocs.io/en/latest/) [[code]](https://github.com/cculianu/electrum-cash-protocol) - ElectrumX Protocol for [fulcrum](https://fulcrumserver.org) (UTXO indexer/SPV service).
- [Electrum Cash Protocol](https://bitcoincash.network/electrum/) [[code]](https://github.com/dagurval/electrum-cash-protocol) - Protocol for SPV clients and servers.
- [Payment Requests Specification (BIP-0070)](https://github.com/bitcoin/bips/blob/master/bip-0070.mediawiki) - For dealing with invoice style payments at specific amounts.
- [Price Oracle](https://gitlab.com/GeneralProtocols/priceoracle/specification) [[implementation]](https://gitlab.com/GeneralProtocols/priceoracle/library) - Price oracle.
- [Memo Protocol](https://memo.cash/protocol) - for the on-chain tweet style social media app.
- [CashShuffle](https://cashshuffle.com/) [[spec]](https://github.com/cashshuffle/spec/blob/master/SPECIFICATION.md) - a privacy protocol for combining transactions with others, splitting to the lowest common amount.

## Discussion

An archive of past and future ideas for Bitcoin Cash ongoing at Bitcoin Cash Research (BCR). Collaborating participants have recorded their thoughts and concerns about various potential ideas & implemented improvements.

- [Bitcoin Cash Research](https://bitcoincashresearch.org/) - Site dedicated to technical discussion.

## CHIP Process

Protocol changes, software standards and application specifications may be proposed by anyone. The recommended process for consensus building and conflict reduction is known as the Cash Improvement Proposal (CHIP) Process.

- [CHIP Guidelines](https://gitlab.com/ggriffith/cash-improvement-proposals/-/blob/master/CHIP-2020-11-CHIP-Guidelines.md)
- [CHIPs: A more detailed process recommendation](https://gitlab.com/im_uname/cash-improvement-proposals/-/blob/master/CHIPs.md)
- [CHIPs](https://bitcoincashresearch.org/c/chips/) - a dynamic list of proposed standards
- [List of CHIPs](https://bch.info/chips) - documents that record proposals to upgrade the Bitcoin Cash protocol, and their ongoing progress, both technical and consensus-building.

### Previous consensus changes, May 2023:

- [CHIP-2021-01 Restrict Transaction Version (v1.0)](https://gitlab.com/bitcoin.cash/chips/-/blob/master/CHIP-2021-01-Restrict%20Transaction%20Versions.md)
- [CHIP-2021-01 Minimum Transaction Size (v0.4)](https://gitlab.com/bitcoin.cash/chips/-/blob/master/CHIP-2021-01-Allow%20Smaller%20Transactions.md)
- [CHIP-2022-02 CashTokens (v2.2.1)](https://github.com/bitjson/cashtokens/)
- [CHIP-2022-05 P2SH32 (v1.5.1)](https://gitlab.com/0353F40E/p2sh32/-/blob/main/CHIP-2022-05_Pay-to-Script-Hash-32_(P2SH32)_for_Bitcoin_Cash.md)

Anyone may propose an improvement to Bitcoin Cash, but the responsibility is on the CHIP owner to see the idea through to fruition and build consensus.

## Bitcoin Script

- 🔵 [Cashscript](https://cashscript.org/docs/basics/about/) [[code]](https://github.com/Bitcoin-com/cashscript) [[playground]](https://playground.cashscript.org/) - a solidity-style language that compiles to Bitcoin Cash Script.
- 🔵 [bitauth ide](https://ide.bitauth.com/) [[code]](https://github.com/bitauth/bitauth-ide) [[video intro]](https://www.youtube.com/watch?v=o-igo-adS8E) - an integrated development environment for bitcoin authentication.
- [AutoCove](https://github.com/TinosNitso/AutoCove-Plugin) - Electrum-cash script decoder.
- [Cashscript VSCode plugin](https://marketplace.visualstudio.com/items?itemName=nathanielcherian.cashscript) [[code]](https://github.com/nathanielCherian/vscode-cashscript) - Visual Studio Code extension for cashscript.

# Software

## Full Nodes

- 🔵 [BCHN](https://bitcoincashnode.org/) [[code]](https://gitlab.com/bitcoin-cash-node/bitcoin-cash-node) [[docs]](https://docs.bitcoincashnode.org/) - a descendant of the Bitcoin Core and Bitcoin ABC software projects with independent development team. C/C++.
- 🔵 [BitcoinUnlimited](https://www.bitcoinunlimited.info/) [[code]](https://github.com/BitcoinUnlimited/BitcoinUnlimited) - a full node implentation focused on supporting user needs, C/C++.
    - [Bitcoin Unlimited Improvement Proposals (BUIPS)](https://www.bitcoinunlimited.info/voting/)
- 🔵 [Flowee the Hub](https://flowee.org/) [[code]](https://codeberg.org/Flowee/thehub) - a node supporting a suite of software focused on payment integration. C++
- 🔵 [Bitcoin Verde](https://bitcoinverde.org/) [[code]](https://github.com/softwareverde/bitcoin-verde) [[docs]](https://explorer.bitcoinverde.org/documentation/) - java implementation with the goal of being interoperable with mining nodes.
- 🔵 [Knuth](https://kth.cash/) [[code]](https://github.com/k-nuth/kth) - a high performance implementation of the Bitcoin protocol focused on applications needing extra capacity and resilience.
- [bchd](https://bchd.cash/) [[code]](https://github.com/gcash/bchd) [[docs]](https://github.com/gcash/bchd/tree/master/docs) - [DEPRECATED] alternative implementation written in Go (golang)

### Developer Resources

- [Bitcoin Cash Research](https://bitcoincashresearch.org/) - Site dedicated to technical research on Bitcoin Cash.

## Open-Source Teams Building on Bitcoin Cash

> If you want to go fast, go alone. If you want to go far, go together.
>
> -- An African Proverb.

There are various groups developing software stacks & apps for the broader ecosystem.

- [General Protocols](https://GeneralProtocols.com) [[repos]](https://gitlab.com/GeneralProtocols) - Team researching and developing protocols for non-custodial and trustless networks using BitBox. (Typescript and Javascript)
- [Electron Cash](https://electroncash.org/) [[repos]](https://github.com/Electron-Cash/) - Team maintaining a desktop SPV wallet with plugins and mobile app (Python)
- [Flowee](https://flowee.org) [[repos]](https://codeberg.org/Flowee) - Team maintaining a non-mining full node and services to access the Bitcoin Cash network. (C++, NodeJs et al)
- [FullStack Cash](https://fullstack.cash/) [[repos]](https://github.com/Permissionless-Software-Foundation) - Team building web/ipfs apps based on BitBox compatible stack. (Javascript)
- [Mainnet Cash](https://mainnet.cash/) [[repos]](https://github.com/mainnet-cash/) - Loose-knit team maintaining a shared server-side and client-side library.

## Simple Payment Verification (SPV)

- 🔵 [Fulcrum](https://fulcrumserver.org) [[repos]](https://github.com/cculianu/Fulcrum/) - A fast & nimble SPV Server for Bitcoin Cash.
- 🔵 [Rostrum](https://gitlab.com/bitcoinunlimited/rostrum) - Rostrum is an efficient implementation of Electrum Server written in Rust.

## Libraries & SDKs

- [Developer tools](https://bch.info/en/developers) - Page devoted to high level developer tools.
- [Mainnet Cash List](https://mainnet.cash/for-developers.html) - A list of useful services for developers.

### Language Agnostic

-  🔵 [mainnet](https://mainnet.cash/) [[tutorial]](https://mainnet.cash/tutorial/) [[rest spec]](https://rest-unstable.mainnet.cash/api-docs/#/) - Typescript library, also available via rest api, or [python](https://github.com/mainnet-cash/mainnet-python-generated), [golang](https://github.com/mainnet-cash/mainnet-go-generated), [php](https://github.com/mainnet-cash/mainnet-php-generated) clients, [et. al](https://mainnet.cash/tutorial/other-languages.html)
- [Insomnia](https://insomnia.fountainhead.cash/) [[code]](https://github.com/fountainhead-cash/insomnia) - Swagger/OpenAPI3 specification for ElectrumX
- [BitBox OpenAPI 3 (Swagger) spec](https://github.com/Bitcoin-com/rest.bitcoin.com/tree/master/swaggerJSONFiles) - for rest.bitcoin.com see: [openapi-generator](https://github.com/OpenAPITools/openapi-generator)

### Typescript

- 🔵 [Libauth](https://libauth.org/) [[code]](https://github.com/bitauth/libauth) - an ultra-lightweight, zero-dependency library for Bitcoin Cash and Bitauth applications. (Formerly `bitcoin-ts`.)
- 🔵 [electrum-cash](https://gitlab.com/electrum-cash) [[docs]](https://electrum-cash.gitlab.io/network/) [[tutorials]](https://read.cash/search?q=electrum-cash) - JavaScript library that lets you connect with one or more Electrum servers.
- [flowee-js](https://flowee.org/floweejs/) [[docs]](https://flowee.org/docs/) [[code]](https://codeberg.org/Flowee/js) - Bindings for using Flowee applications and libraries with the NodeJS JavaScript engine.
- 🔵 [mainnet-js](https://mainnet.cash/) [[code]](https://github.com/mainnet-cash/mainnet-js) - Typescript library, also available over rest.
- [`<qr-code>`](https://github.com/bitjson/qr-code) [[demo]](https://qr.bitjson.com/) – A no-framework, no-dependencies, customizable, animate-able, SVG-based `<qr-code>` HTML element.

### Javascript

- [bch-js](https://github.com/Permissionless-Software-Foundation/bch-js) [[docs]](https://bchjs.fullstack.cash/) - JavaScript library for creating web and mobile apps that can interact with the Bitcoin Cash (BCH) and eCash (XEC) blockchains
- [electrum-cli](https://github.com/rkalis/electrum-cli) - Super simple command line electrum client.
- [bitcore-lib-cash](https://github.com/bitpay/bitcore/tree/master/packages/bitcore-lib-cash) - javaScript library, maintained by bitpay.

### Python

- 🔵 [bitcash](https://pybitcash.github.io/bitcash/) [[code]](https://github.com/pybitcash/bitcash) [[docs]](https://bitcash.dev) - python3 library.
- [jtoomim/p2pool](https://github.com/jtoomim/p2pool) - jtoomim fork of bitcoin pool mining software.

### Rust

- 🔵 [rust-bitcoincash](https://gitlab.com/rust-bitcoincash/rust-bitcoincash/) - Rust Bitcoin Cash library.

### Java

- [bitcoincashj](https://github.com/pokkst/bitcoincashj) - Bitcoin Cash library for Java

### C

- [Breadwallet Core](https://github.com/breadwallet/breadwallet-core) - SPV bitcoin C library.

### PHP

- [cashp](https://github.com/Ekliptor/cashp) - Library for BCH.

### R

- [rbch](https://cran.r-project.org/package=rbch) - Extraction and Statistical Analysis of Data from the BCH Blockchain

# Endorsements

Below is a list of endorsements made in the [Chip Process](#chip-process) in reverse chronological order.

## The [Adaptive Blocksize Limit Algorithm (ebaa) CHIP](https://gitlab.com/0353F40E/ebaa) for the May 2024 BCH Upgrade is AWESOME!

[a42f44791b343ffcc118b0dd6645972e9a165e83](https://gitlab.com/0353F40E/ebaa/-/commit/a42f44791b343ffcc118b0dd6645972e9a165e83)


## The [CashTokens](https://bitcoincashresearch.org/t/chip-2022-02-cashtokens-token-primitives-for-bitcoin-cash/725) and [P2SH32 CHIP](https://bitcoincashresearch.org/t/chip-2022-05-pay-to-script-hash-32-p2sh32-for-bitcoin-cash/806) Proposals for the May 2023 BCH Upgrade are AWESOME!

[539b2a492002da881a9ef9aa6604327299c7a498](https://github.com/bitjson/cashtokens/commit/539b2a492002da881a9ef9aa6604327299c7a498)



# The Archive

Due to the nature of bitcoin, some stuff is forever...

- [chaintip](https://www.chaintip.org) - An on-chain non-custodial tipping bot for reddit/twitter & github. [DEPRECATED due to reddit API access changes]

## Bitcoin Script tools

- [spedn](https://spedn.pl/) [[code]](https://bitbucket.org/o-studio/spedn/src/develop/) [[docs]](https://spedn.readthedocs.io/en/latest/) - a high level smart contract language that compiles to Bitcoin Cash Script.
- [meep](https://github.com/gcash/meep) - a command line Bitcoin Cash script debugger.

## Simple Ledger Protocol (SLP Token)

The Permissionless Software Foundation is actively maintaining an SLP wallet and indexer, denoted with starts (⭐) below. 

### Protocols

- Simple Ledger Protocol (SLP) [[specs]](https://slp.dev) - for handling ERC-20 style tokens.
- [Simple Ledger Postage Protocol](https://github.com/simpleledger/slp-specifications/blob/master/slp-postage-protocol.md) - Protocol for sending SLP tokens without BCH "gas".

### Libraries

- **⭐ SLP Indexer ⭐** [[code]](https://github.com/Permissionless-Software-Foundation/psf-slp-indexer) - Functional SLP token indexer running token infrastructure for several businesses. 
- Simple Ledger [[repos]](https://github.com/simpleledger) - Group leading SLP token integration. (Typescript & Python)
- [SLP Explorer](https://simpleledger.info/) [[code]](https://github.com/salemkode/slp-explorer) [[backend src]](https://github.com/salemkode/slp-explorer-backend) - Slp explorer for bitcoin cash.
- SLPDB [[code]](https://github.com/simpleledger/SLPDB) [[doc]](https://slp.dev/tooling/slpdb/) - simpleledger indexer
- [gs++](https://gs.fountainhead.cash/) [[code]](https://github.com/blockparty-sh/cpp_slp_graph_search) [[doc]](https://gs.fountainhead.cash/swagger.html) - a fast SLP indexer, validator, and graph search server.
- [SLP Stream](https://slpstream.fountainhead.cash/channel) [[code]](https://github.com/blockparty-sh/slpstream) [[doc]](https://slp.dev/tooling/slpstream/) - a frontend API for GS++ that provides a streaming output of new transactions.
- [goslp](https://github.com/simpleledgerinc/goslp) - SLP go libraries.
- [SLP Indexer](https://github.com/Bitcoin-com/slp-indexer) - bitcoin.com indexer.
- [SLP Icons](https://github.com/kosinusbch/slp-token-icons) - Hosted icons for slp tokens.

## SLP Token Projects

- **⭐ [PSF wallet](https://wallet.fullstack.cash/) ⭐** [[code]](https://github.com/Permissionless-Software-Foundation/gatsby-ipfs-web-wallet) - An web wallet with SLP support.
- [SLP Explorer](https://simpleledger.info/) [[code]](https://github.com/salemkode/slp-explorer) [[backend src]](https://github.com/salemkode/slp-explorer-backend) - Open source explorer for SLP tokens.
- Electron-Cash SLP Edition [[code]](https://github.com/simpleledger/Electron-Cash-SLP) [[releases]](https://github.com/simpleledger/Electron-Cash-SLP/releases)
- Honk Token [[archive]](https://web.archive.org/web/20230921212507/https://honk.cash/) [[whitepaper]](https://web.archive.org/web/20220409174235/https://www.honk.cash/whitepaper.pdf) - A gambling/gaming/multipurpose SLP token.
- mistcoin [[archive]](http://web.archive.org/web/20210128134553/https://mistcoin.org/) [[blue miner]](https://gitlab.com/blue_mist/miner) - A mineable SLP token using a proof-of-work covenant contract
- SpiceToken [[archive]](https://web.archive.org/web/20230216030610/https://spicetoken.org/) - A meme SLP token for social tipping.


tons of data from awesome monero:

# Awesome Monero List

A curated list of awesome Monero libraries, tools, and resources.

## Contents

- [Resources](#resources)
- [Wallets](#wallets)
- [Libraries](#libraries)
- [Docker](#docker)
- [Tools](#tools)
- [Nodes](#nodes)
- [Blockchain Explorers](#blockchain-explorers)
- [Built with Monero](#build-with-monero)
- [Mining](#mining)
- [Decentralized Exchanges](#decentralized-exchanges)
- [Atomic Swaps](#atomic-swaps)
- [Integrations](#integrations)
- [Merchants](#merchants)
- [Point of Sale](#point-of-sale)
- [Future development](#future-development)
- [Other](#other)

## Resources

- [Official Website](https://getmonero.org/)
- [Official GitHub](https://github.com/monero-project/monero)
- [Official Twitter](https://twitter.com/monero)
- [Official Reddit](https://www.reddit.com/r/Monero/)
- [Unofficial Docs](https://docs.monero.study/)
- [Monero Research Lab](https://github.com/monero-project/research-lab)

- [Implementing Seraphis](https://raw.githubusercontent.com/UkoeHB/Seraphis/master/implementing_seraphis/Impl-Seraphis-0-0-2.pdf)
- [RandomX](https://github.com/tevador/RandomX) - RandomX is a proof-of-work (PoW) algorithm that is optimized for general-purpose CPUs.
- [LMDB](https://github.com/LMDB/lmdb) - Lightning Memory-Mapped Database

### Books

- [Mastering Monero](https://github.com/monerobook/monerobook) - "Mastering Monero: The future of private transactions" is your guide through the world of Monero, a leading cryptocurrency with a focus on private and censorship-resistant transactions. This book contains everything you need to know to start using Monero in your business or day-to-day life, even if you've never understood or interacted with cryptocurrencies before.
- [monero-book](https://github.com/Cuprate/monero-book) - This book aims to document the Monero protocol. Currently, work is being done to document Monero's consensus rules. This being completed as a part of [Cuprate](https://github.com/Cuprate/cuprate), the Rust Monero node. ([Website](https://monero-book.cuprate.org/))

## Wallets

### Desktop Wallets

- [Monero GUI Wallet](https://getmonero.org/downloads/) - Official desktop wallet
- [Feather Wallet](https://github.com/feather-wallet/feather) ([Website](https://featherwallet.org/)) - Lightweight desktop wallet
- [monero-wallet-generator](https://github.com/moneromooo-monero/monero-wallet-generator) - Self contained offline javacsript Monero wallet generator
- [Cake Wallet](https://github.com/cake-tech/cake_wallet) - Popular iOS and Android wallet and desktop wallet

### Mobile Wallets

- [Cake Wallet](https://github.com/cake-tech/cake_wallet) - Popular iOS and Android wallet and desktop wallet
- [Monerujo](https://github.com/m2049r/xmrwallet) - Popular Android wallet
- [Stack Wallet](https://github.com/cypherstack/stack_wallet) - A multicoin, cryptocurrency wallet 
- [ANONERO](http://anonero.io/) - Hardened wallet with enforced privacy & security for Android (onion link)
- [MYSU](http://rk63tc3isr7so7ubl6q7kdxzzws7a7t6s467lbtw2ru3cwy6zu6w4jad.onion/) - A no-bullshit, pure Monero wallet suitable for both newcomers and experienced users. For Android. (onion link)

### Hardware Wallets

- [Kastelo](https://github.com/monero-project/kastelo) - This is the project to create an official Monero Hardware Wallet (Dead project)
- [passport2-monero](https://github.com/mjg-foundation/passport2-monero) - v2.x.x series of firmware for Passport, rebuilt for monero 
- [MoneroSigner](https://github.com/Monero-HackerIndustrial/MoneroSigner) - Seedsigner Monero fork. Use an air-gapped Raspberry Pi Zero to sign monero transactions!
- [Monero Ledger App](https://github.com/LedgerHQ/app-monero) - Monero wallet application for Ledger Nano S and Nano X. (avoid buying Ledger products)

### Other Wallets
- [Monero Subscriptions Wallet](https://github.com/lukeprofits/Monero_Subscriptions_Wallet) - A Monero wallet that automatically pays subscriptions.

## Libraries

- [monero-ts](https://github.com/woodser/monero-ts) - Monero TypeScript library for Node.js and browsers
- [monerophp](https://github.com/monero-integrations/monerophp) - A Monero library written in PHP by the Monero Integrations team.
- [monero-python](https://github.com/monero-integrations/monero-python) -  A comprehensive Python module for handling Monero cryptocurrency
- [monero-rpc-php](https://github.com/refring/monero-rpc-php) - Monero daemon and wallet RPC client library written in modern PHP.
- [monero-java](https://github.com/woodser/monero-java) - Java library for using Monero 
- [monero-rs](https://github.com/monero-rs/monero-rs) - Library with support for de/serialization on block data structures and key/address generation and scanning related to Monero cryptocurrency.
- [libmonero](https://github.com/monumexyz/libmonero) - libmonero is a library for the Monero cryptocurrency written in Rust. It is designed to be fast, safe and easy to use.
- [monero-cpp](https://github.com/woodser/monero-cpp) - C++ library for using Monero
- [go-monero-rpc-client](https://github.com/omani/go-monero-rpc-client) - A go client for the Monero wallet and daemon RPC
- [go-monero](https://github.com/duggavo/go-monero) - A multi-platform Go library for interacting with Monero servers either on clearnet or not, supporting daemon and wallet RPC, p2p commands and ZeroMQ.

## Docker

- [Simple Monerod Docker](https://github.com/sethforprivacy/simple-monerod-docker) - A simple docker image for running a Monero node.
- [Monero Suite](https://github.com/hundehausen/monero-suite) ([Website](https://monerosuite.org)) - Build your personal docker-compose.yml file for Monero services. 
- [Docker-XMRig](https://github.com/metal3d/docker-xmrig) - Xmrig containeried to mine monero cryptocurrency
- [Moneroblock Docker](https://github.com/sethforprivacy/moneroblock-docker) - A simple and straightforward Dockerized MoneroBlock built from source and exposing standard ports.

## Tools

- [Monero Inflation Checker](https://github.com/DangerousFreedom1984/monero_inflation_checker) - Minimal Python tools and educational material for checking inflation in Monero. You can get more information at moneroinflation.com.
- [Monero Vanity Address Generator](https://github.com/hinto-janai/monero-vanity) - Monero vanity address generator for CPUs
- [monero-lws](https://github.com/vtnerd/monero-lws) - Monero Light Wallet Server (scans monero viewkeys and implements mymonero API)

## Nodes

- [Monero Node List](https://moneroworld.com/) - A list of public Monero nodes.
- [Monero Node Scanner](https://monerohash.com/nodes-distribution.html) - A tool to scan the Monero network for nodes.
- [monero.fail](https://monero.fail/) -  Monero public node aggregator.
- [Monerod-in-Termux](https://github.com/CryptoGrampy/android-termux-monero-node) - Run a Monero Node on Android using Termux
- [check-monero-seed-nodes](https://github.com/plowsof/check-monero-seed-nodes) - A script to check the status of Monero seed nodes
- [Monero Node for Umbrel](https://github.com/deverickapollo/umbrel-monero) - Run a Monero node on your Umbrel personal server.
- [xmr.sh](https://github.com/vdo/xmr.sh) - xmr.sh script wizard sets up a new server running a monero node daemon with Docker compose, with your choice of SSL certificates for your domain, network selection, a Tor hidden service, Grafana dashboard and more.
- [Monero Nodo](https://github.com/MoneroNodo/Nodo) - Software running on a [Monero Nodo](https://moneronodo.com/): Monero Full Node on powerful hardware

## Blockchain Explorers

- [Onion Monero Blockchain Explorer](https://github.com/moneroexamples/onion-monero-blockchain-explorer) - A Monero blockchain explorer.
- [Moneroblock](https://github.com/duggavo/MoneroBlock) -  Decentralized and trustless Monero block explorer

## Built with Monero

- [Nerostr](https://github.com/pluja/nerostr) -  nostr paid relay, but with monero
- [NEVEKO](https://github.com/creating2morrow/neveko) - full-stack privacy application with gpg messaging, monero multisig and built-in i2p marketplace
- [Split My Lunch](https://github.com/AlexAnarcho/split-my-lunch) - Allow co-workers to split the lunch bill in Monero
- [XMR-T3-starter](https://gitlab.com/monero-studio/xmr-t3-starter) - A starter template for a T3 web app with monero-ts. t3-stack: nextjs (react), typescript, tailwind, trpc, prisma also includes: shadcn/ui, monero-ts

## Mining

- [XMRig](https://github.com/xmrig/xmrig) - High performance, open source, cross platform RandomX, CryptoNight and Argon2 CPU/GPU miner
- [Gupax](https://github.com/hinto-janai/gupax) - A simple GUI for mining Monero on P2Pool, using XMRig.
- [P2Pool](https://github.com/SChernykh/p2pool) - P2Pool is a decentralized Monero mining pool that works by creating a peer-to-peer network of miner nodes.
- [XMRig Proxy](https://github.com/xmrig/xmrig-proxy) - Stratum proxy with Web interface, support for several backup pools, and more.
- [Docker-XMRig](https://github.com/metal3d/docker-xmrig) - Xmrig containeried to mine monero cryptocurrency
- [MoneroOS](https://github.com/4rkal/MoneroOS) - Plug and play monero mining archuseriso config
- [XMRig for Android](https://github.com/XMRig-for-Android/xmrig-for-android) - ⛏ Mine Monero from your android device 

## Decentralized Exchanges

- [Bisq](https://github.com/bisq-network/bisq) ([Website](https://bisq.network/)) - A decentralized exchange network for trading Monero and other cryptocurrencies.
- [Haveno](https://github.com/haveno-dex/haveno) - A decentralized, peer-to-peer, non-custodial Monero exchange for trading fiat currencies for Monero.
- [Serai](https://github.com/serai-dex/serai) - Serai is a new DEX, built from the ground up, initially planning on listing Bitcoin, Ethereum, DAI, and Monero, offering a liquidity-pool-based trading experience. Funds are stored in an economically secured threshold-multisig wallet.
- [BasicSwapDex](https://github.com/tecnovert/basicswap) ([Website](https://basicswapdex.com/)) - The BasicSwap DEX is a privacy-first and decentralized exchange which features cross-chain atomic swaps and a distributed order book.

## Atomic Swaps

- [XMR to BTC Atomic Swap](https://github.com/comit-network/xmr-btc-swap) - Bitcoin–Monero Cross-chain Atomic Swap
- [ETH-XMR Atomic Swaps](https://github.com/AthanorLabs/atomic-swap) - 💫 ETH-XMR atomic swap implementation
- [UnstoppableSwap GUI](https://github.com/UnstoppableSwap/unstoppableswap-gui) - Graphical User Interface (GUI) For Trustless Cross-Chain XMR<>BTC Atomic Swaps
- [BCH-XMR-SWAP PoC](https://github.com/PHCitizen/bch-xmr-swap) - A proof of concept for a Bitcoin Cash to Monero atomic swap
- [Farcaster Project](https://github.com/farcaster-project) - Farcaster is a cross-chain atomic swap protocol and implementation who allows to exchange Bitcoin and Monero in a peer-to-peer manner with anyone running a Farcaster node.
- [Samourai XMR-BTC Swap Beta](https://code.samourai.io/wallet/comit-swaps-java) - A GUI for COMIT XMR-BTC atomic swaps with modifications to further enhance anonymity, with the Automated Swap Backend (ASB) built-in, as well as Samourai Wallet Whirlpool for automatic mixing of redeemed BTC. (Beta!)


## Merchants

- [Monero Merchants](https://www.monerooutreach.org/stories/monero_merchants.html) - A list of merchants that accept Monero as payment.
- [Monerica](https://github.com/monerica-project/monerica) ([Website](https://monerica.com/)) - A directory for a Monero circular economy
- [Monero for Merchants](https://github.com/ASchmidt1024/monero-for-merchants-booklet) - A printable booklet to attract merchants to accept Monero (multiple languages!)

## Point of Sale

- [Kasisto](https://github.com/amiuhle/kasisto) - A Monero Point of Sale payment system 
- [Monero Gateway for WooCommerce](https://github.com/monero-integrations/monerowp) - A Monero WooCommerce Plugin for Wordpress 
- [MoneroPay](https://github.com/moneropay/moneropay) - A Monero payment gateway for WooCommerce
- [Monero Merchant](https://github.com/RuiSiang/monero-merchant) - Monero Merchant is a RESTful API wrapper for the official Monero wallet RPC. This project is mainly for merchants who hope to accept Monero as payment.
- [AcceptXMR](https://github.com/busyboredom/acceptxmr) - This library aims to provide a simple, reliable, and efficient means to track monero payments.
- [HotShop](https://github.com/CryptoGrampy/HotShop) - An Ephemeral, browser-based, no-private-key, no-server Point of Sale for receiving and validating Monero payments. Repository is archived :(
- [monerochan-merchant-rpc](https://github.com/spirobel/monerochan-merchant-rpc) - A tool to accept digital cash at your online business.

## Future development

- [Seraphis](https://github.com/UkoeHB/Seraphis) - Seraphis is a privacy-focused transaction protocol for p2p electronic cash systems (e.g. cryptocurrencies).
- [Full chain membership proofs](https://github.com/kayabaNerve/full-chain-membership-proofs)
- [Cuprate](https://github.com/Cuprate/cuprate) - an upcoming experimental, modern & secure monero node. Written in Rust.
- [wallet3](https://github.com/seraphis-migration/wallet3) - Info and discussions about a hypothetical full 'wallet2' rewrite from scratch 
