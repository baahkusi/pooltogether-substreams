# PooltogetherV4 Substreams

- This repo contains example substream implementation for pooltogether set of smart contracts.

- This is mainly for demonstration purposes but should still work.

- Pooltogether is a collection of smart contracts https://dev.pooltogether.com/protocol/V4/deployments/mainnet.

- Each folder focuses on the substream for each contract.

## Prize DistributorV4

- Read more about it here https://dev.pooltogether.com/protocol/V4/contracts/v4-core/PrizeDistributor

- There is also a subgraph for it here https://github.com/pooltogether/v4-prizes-subgraph/tree/main

- This substream implementation will implement the existing subgraph entities in addition to extra ones.

### Build & Pack

- `make pack` to create `.spkg` file.


### Deploy on hosted subgraph studio

- create a subgraph on the graph website https://thegraph.com/explorer

- you will be given a list of steps to follow, you only need to follow these;

- `graph auth --studio [studio-key]` to authenticate

- `graph deploy --studio [studio-name]`

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