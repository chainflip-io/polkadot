window.SIDEBAR_ITEMS = {"constant":[["AVERAGE_ON_INITIALIZE_RATIO","We assume that an on-initialize consumes 1% of the weight on average, hence a single extrinsic will not be allowed to consume more than `AvailableBlockRatio - 1%`."],["BABE_GENESIS_EPOCH_CONFIG","The BABE epoch configuration at genesis."],["MAXIMUM_BLOCK_WEIGHT","We allow for 2 seconds of compute with a 6 second average block time. The storage proof size is not limited so far."],["NORMAL_DISPATCH_RATIO","We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by  Operational  extrinsics."],["VERSION","Runtime version (Kusama)."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["EPMCall","Contains one variant per dispatchable that can be called by an extrinsic."],["HoldReason","A reason for placing a hold on funds."],["HoldReasonNis","A reason for the NIS pallet placing a hold on funds."],["OriginCaller",""],["ProxyType","The type used to represent the kinds of proxying allowed."],["RuntimeCall",""],["RuntimeEvent",""],["StakerStatus","Indicates the initial status of the staker."],["SystemCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"fn":[["native_version","Native version."]],"macro":[["impl_elections_weights","Implements the weight types for the elections module and a specific runtime. This macro should not be called directly; use [`impl_runtime_weights`] instead."]],"mod":[["api",""],["governance","New governance configurations for the Kusama runtime."],["private",""],["xcm_config","XCM configurations for the Kusama runtime."]],"struct":[["AnnouncementDepositBase",""],["AnnouncementDepositFactor",""],["BagThresholds",""],["BaseFilter","We currently allow all calls."],["BasicDeposit",""],["BetterUnsignedThreshold",""],["BlockExecutionWeight","Time to execute an empty block. Calculated by multiplying the Average with `1.0` and adding `0`."],["BlockWeights","Block weights base values and limits."],["BondingDuration",""],["BountyDepositBase",""],["BountyDepositPayoutDelay",""],["BountyUpdatePeriod",""],["BountyValueMinimum",""],["Burn",""],["CandidateDeposit",""],["ChallengePeriod",""],["ChildBountyValueMinimum",""],["ConfigDepositBase",""],["CrowdloanId",""],["CuratorDepositMax",""],["CuratorDepositMin",""],["CuratorDepositMultiplier",""],["DataDepositPerByte",""],["DepositBase",""],["DepositFactor",""],["EndingPeriod",""],["EpochDuration",""],["EraPayout",""],["ExistentialDeposit",""],["ExpectedBlockTime",""],["ExtrinsicBaseWeight","Time to execute a NO-OP extrinsic, for example `System::remark`. Calculated by multiplying the Average with `1.0` and adding `0`."],["FieldDeposit",""],["FirstMessageFactorPercent",""],["FriendDepositFactor",""],["GenesisConfig",""],["ImOnlineUnsignedPriority",""],["IndexDeposit",""],["IntakePeriod",""],["LeasePeriod",""],["MaxActiveChildBountyCount",""],["MaxActiveValidators","Setup election pallet to support maximum winners upto 2000. This will mean Staking Pallet cannot have active validators higher than this count."],["MaxAdditionalFields",""],["MaxApprovals",""],["MaxAuthorities",""],["MaxCandidateIntake",""],["MaxElectableTargets","… and all of the validators as electable targets. Whilst this is the case, we cannot and shall not increase the size of the validator intentions."],["MaxElectingVoters","We take the top 12500 nominators as electing voters.."],["MaxFriends",""],["MaxIntakeWeight",""],["MaxKeys",""],["MaxLockDuration",""],["MaxLocks",""],["MaxMemoLength",""],["MaxNominations",""],["MaxNominatorRewardedPerValidator",""],["MaxPeerDataEncodingSize",""],["MaxPeerInHeartbeats",""],["MaxPending",""],["MaxPointsToBalance",""],["MaxProxies",""],["MaxRegistrars",""],["MaxReserves",""],["MaxScheduledPerBlock",""],["MaxSetIdSessionEntries",""],["MaxSignatories",""],["MaxStrikes",""],["MaxSubAccounts",""],["MaximumReasonLength",""],["MaximumSchedulerWeight",""],["MinBid",""],["MinContribution",""],["MinReceipt",""],["MinVestedTransfer",""],["MinimumPeriod",""],["NisBasePeriod",""],["NisHoldReason",""],["NisPalletId",""],["NisTarget",""],["NoPreimagePostponement",""],["NominationPoolsMigrationV4OldPallet",""],["NposCompactSolution24",""],["NposSolutionPriority",""],["OffchainRepeat",""],["OffchainSolutionLengthLimit","A limit for off-chain phragmen unsigned solution length."],["OffchainSolutionWeightLimit","A limit for off-chain phragmen unsigned solution submission."],["OffendingValidatorsThreshold",""],["OldSubmissionDeposit",""],["OnChainSeqPhragmen",""],["OperationalFeeMultiplier","This value increases the priority of `Operational` transactions by adding a “virtual tip” that’s equal to the `OperationalFeeMultiplier * final_fee`."],["OriginPrivilegeCmp","Used the compare the privilege of an origin inside the scheduler."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["ParaDeposit",""],["ParasUnsignedPriority",""],["ParityDbWeight","`ParityDB` can be enabled with a feature flag, but is still experimental. These weights are available for brave runtime engineers who may want to try this out as default."],["PeriodSpend",""],["PoolsPalletId",""],["Prefix",""],["PreimageBaseDeposit",""],["PreimageByteDeposit",""],["ProposalBond",""],["ProposalBondMaximum",""],["ProposalBondMinimum",""],["ProxyDepositBase",""],["ProxyDepositFactor",""],["RecoveryDeposit",""],["RemoveKeysLimit",""],["ReportLongevity",""],["RocksDbWeight","By default, Substrate uses `RocksDB`, so this will be the weight used throughout the runtime."],["RotationPeriod",""],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeOrigin","The runtime origin type representing the origin of a call."],["SS58Prefix",""],["SampleLength",""],["SessionKeys",""],["SessionsPerEra",""],["SignedDepositBase",""],["SignedDepositByte",""],["SignedMaxRefunds",""],["SignedMaxSubmissions",""],["SignedPhase",""],["SignedRewardBase",""],["SlashDeferDuration",""],["SocietyPalletId",""],["SpendPeriod",""],["SubAccountDeposit",""],["ThawThrottle",""],["TipCountdown",""],["TipFindersFee",""],["TipReportDepositBase",""],["TransactionByteFee",""],["TreasuryPalletId",""],["UnsignedPhase",""],["UnvestedFundsAllowedWithdrawReasons",""],["Version",""],["WrongSideDeduction",""]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["Address","The address format for describing accounts."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["Auctions",""],["AuthorityDiscovery",""],["AuthorityDiscoveryConfig",""],["Authorship",""],["Babe",""],["BabeConfig",""],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","`BlockId` type as expected by this runtime."],["Bounties",""],["ChildBounties",""],["Claims",""],["ClaimsConfig",""],["Configuration",""],["ConfigurationConfig",""],["ConvictionVoting",""],["Crowdloan",""],["Dmp",""],["ElectionProviderMultiPhase",""],["Executive","Executive: handles dispatch to the various modules."],["FastUnstake",""],["FellowshipCollective",""],["FellowshipReferenda",""],["Grandpa",""],["GrandpaConfig",""],["Header","Block header type as expected by this runtime."],["Historical",""],["Hrmp",""],["HrmpConfig",""],["Identity",""],["ImOnline",""],["ImOnlineConfig",""],["Indices",""],["IndicesConfig",""],["Initializer",""],["Migrations","All migrations that will run on the next runtime upgrade."],["Multisig",""],["Nis",""],["NisCounterpartBalances",""],["NisCounterpartBalancesConfig",""],["NominationPools",""],["NominationPoolsConfig",""],["Offences",""],["Origins",""],["ParaInclusion",""],["ParaInherent",""],["ParaScheduler",""],["ParaSessionInfo",""],["ParachainsOrigin",""],["Paras",""],["ParasConfig",""],["ParasDisputes",""],["ParasShared",""],["ParasSlashing",""],["Preimage",""],["Proxy",""],["Recovery",""],["Referenda",""],["Registrar",""],["Scheduler",""],["Session",""],["SessionConfig",""],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The `SignedExtension` to the basic transaction logic."],["SignedPayload","The payload being signed in the transactions."],["Slots",""],["Society",""],["Staking",""],["StakingConfig",""],["System",""],["SystemConfig",""],["Timestamp",""],["TransactionPayment",""],["Treasury",""],["TreasuryConfig",""],["Ump",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Utility",""],["Vesting",""],["VestingConfig",""],["VoterList",""],["Whitelist",""],["XcmPallet",""],["XcmPalletConfig",""]]};