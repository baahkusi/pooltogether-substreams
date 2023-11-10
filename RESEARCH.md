# Introduction

Subgraphs are the decentralized data querying layer of the blockchain powered by The Graph protocol https://thegraph.com/.
The Graph has released a powerful tool called substreams which allows for much more flexibility than subgraphs. This document is a research
on substream powered subgraphs, use cases and analysis of existing projects. Check ./README.md for an implementation of this on the pooltogether protocol https://pooltogether.com/.


## Substream Docs & Example Projects.

- Substream Docs - https://substreams.streamingfast.io/ 
- Substream Powered Subgraphs
    - https://thegraph.com/docs/en/cookbook/substreams-powered-subgraphs/
    -https://thegraph.com/docs/en/developing/substreams-powered-subgraphs-faq/

### Example Projects & Resources

- https://github.com/pinax-network/substreams-cookbook
- https://github.com/matstyler/aave-substreams
- https://github.com/Graph-BuildersDAO/substreams/tree/master/chainlink-prices
- https://github.com/jmulq/ens-substream
- https://github.com/dapplooker/dapplooker-substreams/tree/main/network_substream
- https://github.com/0xPlaygrounds/wsteth-gbd-bounty
- https://github.com/MercuricChloride/erc721-subgraph
- https://github.com/itsjerryokolo/apecoin_substreams
- https://github.com/pinax-network/awesome-substreams


## How do substreams work

- There is the firehose node https://firehose.streamingfast.io/.
- Then there is the substream engine which can run substreams (subsreams are arbitrary processing logic written in rust.).
- And finally there is the arbitrary persistent storage solution.
- This means that blockchain data processed with substreams can be stored anywhere you wish.
- A subgraph powered substream is just a substream which uses subgraph storage as persistent layer.


### How do substream powered subgraphs work.

- First you need to define your substream
- Then you define your subgraph and reference the substream package as datasource for he subgraph.
- As long as the substream output data entities (`graph_out`) is compatible with the subgraph schema entities defined, you have a substream powered subgraph.
- Finally you can deploy and start consumption with graphql just like subgraphs.