import { Text, u32, Enum, getTypeRegistry } from "@polkadot/types";
import { BlockNumber, Balance } from "@polkadot/types/interfaces";
import { MemberId } from "./members";

export type VotingResults = {
  abstensions: u32;
  approvals: u32;
  rejections: u32;
  slashes: u32;
};

export type ProposalParameters = {
  // During this period, votes can be accepted
  votingPeriod: BlockNumber;

  /* A pause before execution of the approved proposal. Zero means approved proposal would be
     executed immediately. */
  gracePeriod: BlockNumber;

  // Quorum percentage of approving voters required to pass the proposal.
  approvalQuorumPercentage: u32;

  // Approval votes percentage threshold to pass the proposal.
  approvalThresholdPercentage: u32;

  // Quorum percentage of voters required to slash the proposal.
  slashingQuorumPercentage: u32;

  // Slashing votes percentage threshold to slash the proposal.
  slashingThresholdPercentage: u32;

  // Proposal stake
  requiredStake: Balance;
};

export type Proposal = {
  parameters: ProposalParameters;
  proposerId: MemberId;
  title: Text;
  description: Text;
  createdAt: BlockNumber;
  status: ProposalStatus;
  votingResults: VotingResults;
};

export const ProposalStatuses: { [key: string]: string } = {
  Active: "Active",
  Cancelled: "Cancelled",
  Expired: "Expired",
  Approved: "Approved",
  Rejected: "Rejected",
  Slashed: "Slashed"
};

export class ProposalStatus extends Enum {
  constructor(value?: any) {
    super(["Active", "Cancelled", "Expired", "Approved", "Rejected", "Slashed"], value);
  }
}

export const VoteKinds: { [key: string]: string } = {
  Abstain: "Abstain",
  Approve: "Approve",
  Reject: "Reject",
  Slash: "Slash"
};

export class VoteKind extends Enum {
  constructor(value?: any) {
    super(["Abstain", "Approve", "Reject", "Slash"], value);
  }
}

export type ProposalVotes = [MemberId, VoteKind][];

export class ProposalId extends u32 {}

const proposalTypes = {
  VoteKind,
  ProposalStatus
};

// export default proposalTypes;
export function registerProposalTypes() {
  try {
    getTypeRegistry().register({
      ProposalId,
      ProposalStatus,
      VoteKind
    });
  } catch (err) {
    console.error("Failed to register custom types of proposals module", err);
  }
}
