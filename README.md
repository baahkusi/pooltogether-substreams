# PooltogetherV4 Substreams

- This repo contains example substream implementation for pooltogether set of smart contracts.

- This is mainly for demonstration purposes but should still work.

- Pooltogether is a collection of smart contracts https://dev.pooltogether.com/protocol/V4/deployments/mainnet.

- Each folder focuses on the substream for each contract.

## Prize DistributorV4

- Read more about it here https://dev.pooltogether.com/protocol/V4/contracts/v4-core/PrizeDistributor

- There is also a subgraph for it here https://github.com/pooltogether/v4-prizes-subgraph/tree/main

- This substream implementation will implement the existing subgraph entities in addition to extra ones.

- The subgraph studio can be found here https://api.studio.thegraph.com/query/52116/pool_prize_distributor/version/latest

- The hosted service can be found here https://api.thegraph.com/subgraphs/name/baahkusi/prize_distributor_stream

### Pack & Build

- `make pack` to create `.spkg` file.

- `graph build`


### Deploy on subgraph studio

- create a subgraph  https://thegraph.com/explorer

- you will be given a list of steps to follow, you only need to follow these;

- `graph auth --studio [studio-key]` to authenticate

- `graph deploy --studio [studio-name]`


### Deploy on hosted service

- create a hosted service account  https://thegraph.com/hosted-service

- create a subgraph from you dashboard,

- you will be given a list of steps to follow, you only need to follow these;

- `graph auth --product hosted-service [studio-key]` to authenticate

- `graph deploy --product hosted-service [studio-name]`


### Sample Query

```graphql
{
  aggregates(first: 5) {
    id
    totalClaimed
  }
  accounts(first: 5) {
    id
    totalClaimed
    draws {
      id
      totalClaimed
      firstClaimedAtTimestamp
      lastClaimedAtTimestamp
    }
  }
}
```