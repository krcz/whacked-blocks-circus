This is a little project I have created during [Whacked Blocks hackaton](https://www.whackedblocks.com/). One of the [challenges to choose from](https://github.com/RampNetwork/whacked-blocks-2018/blob/master/challenges.md) was creating oracle with data from Polish parliament.

I came with an idea of on-chain bets will-the-parliament-member-change-their-party. That would require an oracle for checking the membership of the party. I meant implemented version to be very simple, working in on-demand mode. In order to use oracle one would have to call one of its methods giving numeric id of the member [as assigned on Polish Parliament webpage](http://sejm.gov.pl/Sejm8.nsf/poslowie.xsp?type=A). Oracle contract would then emit an event that would be read by an off-chain program via Web3 API. It would check membership data on the page and put it back on the chain and the oracle contract would call the requesting contract callback method with the result.

Before the event I had a little experience in Rust (10-20 hours) and no knowledge of Solidity. In 10 hours I had managed to create contracts and a program to obtain the events from chain, using the event to obtain required data from the parliament webpage, everything in async way. The only part left would be putting the data back onto the chain. Implementing it would take me 1 more hour I think, but for now I'm putting the code as I have finished it during the hackaton.

The project was named circus, after the shape of [Polish Parliamend building](https://pl.wikipedia.org/wiki/Kompleks_budynk%C3%B3w_Sejmu_Rzeczypospolitej_Polskiej#/media/File:Sejm_RP.jpg) shape.

Libraries I used:
* [Hyper](https://github.com/hyperium/hyper) as HTTP client,
* [select](https://github.com/utkarshkukreti/select.rs) to extract data from HTML,
* [rust-web3](https://github.com/tomusdrw/rust-web3) to communicate with Ethereum nodes,
* [ethabi](https://github.com/paritytech/ethabi) (by Parity Technologies) to decode raw Ethereum messages.

The code is quite messy & hacky (it was a hackaton after all, duh), has hardcoded address of contract uploaded to Ropsten test network, some things in Rust are probably not done the best way, the oracle isn't equiped to make user pay for the callback gas. But it works, and I learned a lot during writing it, mission fucking accomplished!

I would like to help hackathon mentors for helping me understanding some Ethereum concepts and showing how to use [Remix](https://remix.ethereum.org/) IDE for contracts.
