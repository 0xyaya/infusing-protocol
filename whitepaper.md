# Whitepaper

## Introduction
The current global consumption pattern is flawed, with almost each product or service contributing to the planet’s degradation. In a sustainable free market, companies with bad impact would compensate to stay competitive without any regulations. To connect the free market and sustainability, good behavior should earn something valuable on the market that the other doesn’t have.
The Infusing Protocol aims to rewrite this equation, transforming consumption into a regenerative force. The first part is the infusing of any consumption act with carbon credits to earn a proof of carbon removal. The second is to bring value for that proof with world applications integration. At the end, bad actors lose competitivity if they don’t infuse their products or services.
While some advocate for degrowth as the solution to climate change, it presents a narrow and somewhat pessimistic perspective. Here, we present a more optimistic approach—one that demonstrates that growth and Earth's regeneration can coexist and are, in fact, the only path forward.

## Voluntary Carbon Market
The Voluntary Carbon Market (VCM) is a dynamic and fast-growing sector within the broader carbon markets landscape. Unlike compliance carbon markets, which are driven by government regulations, the VCM operates on a voluntary basis, with organizations and individuals voluntarily taking action to mitigate their carbon footprints.
In the VCM, carbon credits, often referred to as Verified Emission Reductions (VERs) or Voluntary Carbon Units (VCUs), are generated through projects that reduce, remove, or avoid greenhouse gas emissions. These projects can range from reforestation and renewable energy initiatives to methane capture from landfills.
Participants in the VCM, known as buyers or voluntary purchasers, acquire these carbon credits to offset their own emissions, demonstrate climate leadership, or align with sustainability goals. The VCM thus provides a flexible and accessible means for entities to take tangible steps toward carbon neutrality or reduced emissions.
This market's growth is driven by increasing awareness of climate change and sustainability, with organizations and individuals seeking to take meaningful climate action beyond regulatory requirements. As a result, the VCM plays a crucial role in global efforts to combat climate change by incentivizing emissions reductions and environmental stewardship on a voluntary basis.

## Solana
Solana is a decentralized, fast and scalable layer one blockchain, with low fees and low latency. It is possible due to key technologies developed by the team like the Proof of History, Gulf Stream the mempool-less transaction forwarding protocol, Sealevel for parallel smart contracts run-time or Cloudbreak the horizontally-scaled accounts database.
Solana’s blockchain architecture relies on accounts for data storage. The program itself is marked as an executable account and is distinct from its own state. In contrast to Ethereum, where smart contracts and data share a common location, Solana’s approach offers enhanced modularity and security.
As a layer one, the single global state bring composability between programs. The Infusing Protocol capitalizes on this composability to seamlessly connect program accounts, creating a comprehensive and efficient carbon credit offsetting system.

## Infused Account
The Infusing Protocol seamlessly associates an “infused score” (measured in tons) with any Solana account that has been infused. This score signifies the quantity of carbon tons reduced or removed for the infused account. This information is recorded on-chain, in an account own by the infusing program and connected to the provided account to infuse. An account can be infused many times to increase the infused score of the based Solana account.
Anything in the world could be tokenized as an account and the infused protocol can infused any account. It mean that the infused protocol could infused the world.

![infused account](https://2535943121-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2F3XtxzbaXegwzDrdmjfcY%2Fuploads%2FGzGSdTX1DVf1lt8jdqgE%2Finfusing.png?alt=media&token=6f0a21ff-d75f-4b25-a61d-d6ad0d2d4bb8 "Infused Account")

## Offset Controller
To mitigate reliance on a single carbon credits provider, the Infusing Protocol routes the payment amount to a strategies controller. This controller effectively allocates the carbon credits for offsetting across various strategies. The allocation is determined based on a percentage allocation state, ensuring diversified and strategic carbon credit distribution. The allocation weight of each strategy should be define by the governance staking. While tokens are staked in allocation weight pools, they will also earn revenues from the infusing protocol fees.

![infused account](https://2535943121-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2F3XtxzbaXegwzDrdmjfcY%2Fuploads%2FWJ8fHFn8FmWB1WxNBgC1%2Foffseting%20controller.png?alt=media&token=1fe57277-7af1-4454-b722-69300fdd0d8b "Infused Account")

## Infused Carbon Token
The Infused Carbon Token (ICT) is the utility token of the Infusing Protocol. Its primary purpose is to facilitate payments within the Infusing Protocol. Additionally, ICT can be staked in strategy allocation pools to determine distribution by the controller. Furthermore, ICT staked in the allocator pools collect fees generated by the Infusing Protocol.
ICT is minted through Carbon Tree NFTs staked in the Carbon Tree staking program. There is no pre-minted supply, and the quantity minted by each NFT is based on the NFT's Infused Score from the Infusing Protocol. This makes the Carbon Tree staking program the first use case of the Infusing Protocol.
​
Tokenomics exemple:
On Solana: 1 epoch =~ 2 days & 1 slot =~ 400 ms
In the initial epoch, 10,000 ICT will be minted and distributed among Carbon Tree stakers based on their Infused Score. The minted amount per epoch will decrease by 10 ICT every epoch to reach a 100 ICT per epoch inflation in ~ 5.5 years. 

## Use cases
Firstly the Infusing Protocol is an easy way to offset tokenized assets to be carbon neutral or even more with regeneration. A first app called The Infusor will transform any NFT on Solana in a eco-friendly version. The protocol tokenomics itself use the infusing mechanism to distribute the ICT inflation. 

Finally, we can see the infusing protocol as a new ReFi primitive to build regenerative applications. Everyone could infused their application assets or rewards infused assets in their algorithms. Per example, a decentralized social network could give more reach and visibility for infused content. Products or services sold could be toknized and infused with a fraction of the total amount to bring sustainability proof to customers.

## Roadmap
​
Firstly, the strategy is to launch the Infusing Protocol small and fast in the Solana community. In that space, people are known to be strong builders and degen NFT traders. Combine, with the ReFi ecosystem, that could create a new culture based on innovations and earth regeneration. That could be close to the Solarpunk movement.

* Core Program 
Build the core infusing program with a simple offsetting strategy. Payment will be exclusively in SOL to avoid tokenmics complexities while also collection SOL-based fees to support infrastructure costs.

* The Infusor App 
Launch a simple app called “The Infusor”, to infused any NFT on Solana with carbon credits and transform it in a eco-friendly version. Here the purpose is to bring early adopters to test and collect feedbacks. A leaderboard of all the infused NFT on Solana with potiential rewards should attract and engage the community.

* ICT, Carbon Tree, Mining
Release the tokenomics mechanism, with the Infused Carbon Token, the Carbon Tree NFT collection and the mining of ICT with the Carbon Tree staking program. Remove SOL payment for ICT.

* Integration time
Create educational content, developers documentations, SDKs to bring more Infusing Protocol integrations and usages.

* Carbon Diversity
Develop new offsetting strategies and allocate the distribution share manually.

* Allocation Pools
Add the ICT staking on allocation pools to collect fees and provide weight on each carbon offsetting strategies.

* Endgame
Consumer app developed by builders and also the Infused Labs to leverage the Infusing Protocol.

* Endgame 
Infused Funds to organize climate events & hackathon
