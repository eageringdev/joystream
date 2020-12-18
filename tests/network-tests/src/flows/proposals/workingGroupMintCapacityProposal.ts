import BN from 'bn.js'
import { Api, WorkingGroups } from '../../Api'
import { VoteForProposalFixture, WorkingGroupMintCapacityProposalFixture } from '../../fixtures/proposalsModule'
import { ProposalId } from '@joystream/types/proposals'
import { assert } from 'chai'
import { FixtureRunner } from '../../Fixture'

export default async function workingGroupMintCapactiy(api: Api, env: NodeJS.ProcessEnv, group: WorkingGroups) {
  const mintCapacityIncrement: BN = new BN(env.MINT_CAPACITY_INCREMENT!)

  // Pre-conditions: members and council
  const council = await api.getCouncil()
  assert(council.length)

  const proposer = council[0].member.toString()
  const newMintCapacity: BN = (await api.getWorkingGroupMintCapacity(group)).add(mintCapacityIncrement)
  const workingGroupMintCapacityProposalFixture: WorkingGroupMintCapacityProposalFixture = new WorkingGroupMintCapacityProposalFixture(
    api,
    proposer,
    newMintCapacity,
    group
  )
  // Propose mint capacity
  await new FixtureRunner(workingGroupMintCapacityProposalFixture).run()

  const voteForProposalFixture: VoteForProposalFixture = new VoteForProposalFixture(
    api,
    workingGroupMintCapacityProposalFixture.getCreatedProposalId() as ProposalId
  )

  // Approve mint capacity
  await new FixtureRunner(voteForProposalFixture).run()
}
