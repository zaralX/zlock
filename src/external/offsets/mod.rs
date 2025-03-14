#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod client {
    pub static mut dwEntityList: usize = 0x0;
    pub static mut dwViewMatrix: usize = 0x0;
    pub static mut dwLocalPlayerController: usize = 0x0;
    pub static mut dwCCitadelCameraManager: usize = 0x0;
    pub static mut dwGlobalVars: usize = 0x0;
    pub static mut dwGameRules: usize = 0x0;
    pub static mut highestEntityIndex: usize = 0x2100;
}

pub mod client_dll {
    // Parent: C_PhysicsProp
    // Field count: 1
    pub mod C_ItemCrate {
        pub const m_eLootType: usize = 0xC88; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CModifier_Synth_Grasp_BulletShield {
        pub const m_fBulletShield: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_RadianceVData {
        pub const m_RadianceFxParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RadianceDamageParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ClientsideDamageParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDamageRecievedSound: usize = 0x8D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Warden_RiotProtocol_CastDelay {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Upgrade_SpiritSnatch_Buff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_StatStealBase {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_TimeWall_Aura {
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_Rutger_ForceField_PushOut {
        pub const m_vStart: usize = 0xC0; //
        pub const m_vDest: usize = 0xCC; //
        pub const m_vCenter: usize = 0xD8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 5
    pub mod CModifier_SiphonBullets_VData {
        pub const m_StealWatcherModifier: usize = 0x668; //
        pub const m_HealModifier: usize = 0x678; //
        pub const m_TracerParticle: usize = 0x688; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeParticle: usize = 0x768; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x848; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_SpellShieldVData {
        pub const m_SpellShieldBuffModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_LeechHealbane_Debuff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_LingeringAssist {
    }
    // Parent: C_BaseTrigger
    // Field count: 0
    pub mod C_TriggerLerpObject {
    }
    // Parent: C_BaseToggle
    // Field count: 0
    pub mod C_FuncMover {
    }
    // Parent: C_PointClientUIWorldPanel
    // Field count: 2
    pub mod CInWorldItemPanel {
        pub const m_hTrackedEntity: usize = 0xAC0; //
        pub const m_nTrackedEntity: usize = 0xAC4; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 7
    pub mod CAbility_Rutger_ForceField_VData {
        pub const m_AuraModifier: usize = 0x1590; //
        pub const m_VictimPushModifier: usize = 0x15A0; //
        pub const m_SlowModifier: usize = 0x15B0; //
        pub const m_strDomeCreated: usize = 0x15C0; //
        pub const m_strChargeUpSound: usize = 0x15D0; //
        pub const m_strPushAndDamage: usize = 0x15E0; //
        pub const m_ChronoSphereChargeParticle: usize = 0x15F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CCitadel_Ability_Tokamak_CrimsonCannonVData {
        pub const m_LaserShot: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChargeParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CasterOnlyTargetParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyTargetedParticle: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strEnemyBeenTargetedSound: usize = 0x1910; //
        pub const m_strCasterTargetSelectedSound: usize = 0x1920; //
        pub const m_strFireSound: usize = 0x1930; //
        pub const m_strImpactSound: usize = 0x1940; //
        pub const m_strBlockedSound: usize = 0x1950; //
    }
    // Parent: CCitadel_Modifier_Base
    // Field count: 0
    pub mod CCitadel_Modifier_MagicCarpet_Shields {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_HollowPoint_Proc {
        pub const m_nStacksPerBullet: usize = 0xC0; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_TechOverflowProcWatcherVData {
        pub const m_BuildUpModifier: usize = 0x668; //
        pub const m_ProcModifier: usize = 0x678; //
        pub const m_BuildupSuccessEffect: usize = 0x688; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: None
    // Field count: 6
    pub mod sky3dparams_t {
        pub const scale: usize = 0x8; //
        pub const origin: usize = 0xC; //
        pub const bClip3DSkyBoxNearToWorldFar: usize = 0x18; //
        pub const flClip3DSkyBoxNearToWorldFarOffset: usize = 0x1C; //
        pub const fog: usize = 0x20; //
        pub const m_nWorldGroupID: usize = 0x88; //
    }
    // Parent: C_BaseTrigger
    // Field count: 1
    pub mod C_TriggerItemShop {
        pub const m_iszSoundName: usize = 0x998; //
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod CFilterName {
        pub const m_iFilterName: usize = 0x5B8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CAbility_Operative_Revelation {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Kobun {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Disruptive_Charge {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Vandal_Pillar {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_Viper_DebuffDaggerVData {
        pub const m_ImpactParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffModifier: usize = 0x1670; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CModifier_CloakingDevice_Active_Ambush_VData {
        pub const m_InvisRevealedParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AmbushParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strActivateAmbushSound: usize = 0x7F8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Tech_Defender_Shredders_Debuff {
    }
    // Parent: C_SoundEventEntity
    // Field count: 1
    pub mod C_SoundEventPathCornerEntity {
        pub const m_vecCornerPairsNetworked: usize = 0x620; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Mirage_SandPhantom_ProcReady_VData {
        pub const m_ProcReadyParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strProcReadySound: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbility_Synth_Blitz_VData {
        pub const m_BlitzModifier: usize = 0x1590; //
        pub const m_TechAmpModifier: usize = 0x15A0; //
        pub const m_SlowParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strProcSound: usize = 0x1690; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Dust_Storm {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ImmobilizeTrap {
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_ChainLightningVData {
        pub const m_TracerParticle: usize = 0x768; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChainModifier: usize = 0x848; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_MagicShield_SpiritBuff {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityMedicHealVData {
        pub const m_HealBeamParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HealTargetParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strHealCastSound: usize = 0x1750; //
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod CCitadelSpectateNode {
    }
    // Parent: CBaseAnimGraph
    // Field count: 1
    pub mod C_Citadel_Ice_Dome_Blocker {
        pub const m_flTurnSolidTime: usize = 0xAF8; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 0
    pub mod CCitadel_Modifier_ItemPickupAuraVData {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_Thumper_BulletWatcherVData {
        pub const m_ExplodeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_Protection_RacketVData {
        pub const m_CastOtherParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ArmorModifier: usize = 0x1670; //
    }
    // Parent: CBodyComponent
    // Field count: 1
    pub mod CBodyComponentSkeletonInstance {
        pub const m_skeletonInstance: usize = 0x80; //
    }
    // Parent: CBaseFilter
    // Field count: 3
    pub mod FilterHealth {
        pub const m_bAdrenalineActive: usize = 0x5B8; //
        pub const m_iHealthMin: usize = 0x5BC; //
        pub const m_iHealthMax: usize = 0x5C0; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 2
    pub mod C_Projectile_Stomp_Projectile {
        pub const m_flWidth: usize = 0x8E0; //
        pub const m_tDieTime: usize = 0x8E4; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadelBaseTriggerAbility {
        pub const m_hAbilityToTrigger: usize = 0xCA0; //
        pub const m_SwappedToTime: usize = 0xCA4; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Bull_Leap_Boosting_Crash {
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbilityPowerSurgeVData {
        pub const m_ChainParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastHitParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffModifier: usize = 0x1750; //
        pub const m_ChainModifier: usize = 0x1760; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_AfterburnWatcherVData {
        pub const m_AfterburnDotModifier: usize = 0x668; //
        pub const m_BuildUpModifier: usize = 0x678; //
        pub const m_ExplodeSound: usize = 0x688; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Item_Bleeding_Bullets_DamageOverTime {
        pub const m_flLastTickTime: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ScalingPowerUp {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_AirDamping {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ShieldGuy_Ability03 {
    }
    // Parent: CCitadel_Modifier_InvisVData
    // Field count: 5
    pub mod CCitadelModifierShadowStepVData {
        pub const m_SilenceModifier: usize = 0x900; //
        pub const m_ArmorDebuff: usize = 0x910; //
        pub const m_InvisChangedEffect: usize = 0x920; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShadowRevealedEffect: usize = 0xA00; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flMinInvisDuration: usize = 0xAE0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PowerSurge {
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_PowerSurgeVData {
        pub const m_TracerParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WeaponFxParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strWeaponShootSound: usize = 0x7F8; //
        pub const m_strBulletWhizSound: usize = 0x808; //
        pub const m_DebuffModifier: usize = 0x818; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 20
    pub mod CCitadel_Ability_ZipLine {
        pub const m_flActivatePressTime: usize = 0x1098; //
        pub const m_bThinking: usize = 0x109C; //
        pub const m_bMoveCollidedPushUp: usize = 0x109D; //
        pub const m_flTimeStartZipping: usize = 0x10AC; //
        pub const m_flTimeForKnockdownProtection: usize = 0x10B0; //
        pub const m_flTimeStopZipping: usize = 0x10B4; //
        pub const m_flCasterSpeed: usize = 0x10B8; //
        pub const m_vecInitialVel: usize = 0x10BC; //
        pub const m_vecAttachPoint: usize = 0x10F0; //
        pub const m_pPrevNode: usize = 0x10FC; //
        pub const m_pNextNode: usize = 0x1100; //
        pub const m_flTimeEnterState: usize = 0x1104; //
        pub const m_flLatchTime: usize = 0x1108; //
        pub const m_flDamagedTime: usize = 0x110C; //
        pub const m_eAttachState: usize = 0x1110; //
        pub const m_iAttachedZipLineLane: usize = 0x1114; //
        pub const m_bDroppedFromZipline: usize = 0x1118; //
        pub const m_hAttachZipLine: usize = 0x1119; //
        pub const m_vAttachZipLineOffset: usize = 0x111C; //
        pub const m_flZiplineAirDrag: usize = 0x1128; //
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 0
    pub mod CCitadel_Ability_SuperNeutralIncendiary {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_TeamRelativeParticleVData {
        pub const m_ParentViewParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_OtherPlayerViewParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_FrenzyAuraVData {
        pub const m_KillModifier: usize = 0x680; //
    }
    // Parent: CCitadel_Modifier_Silenced
    // Field count: 0
    pub mod CCitadel_Modifier_Targeted_Silence_Debuff {
    }
    // Parent: C_ParticleSystem
    // Field count: 4
    pub mod C_TeamRelativeParticleSystem {
        pub const m_iszFriendlyEffectName: usize = 0xE20; //
        pub const m_iszEnemyEffectName: usize = 0xE28; //
        pub const m_iFriendlyEffectIndex: usize = 0xE30; //
        pub const m_iEnemyEffectIndex: usize = 0xE38; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CGameModifier_PlayEffectOnDeath {
        pub const m_sEffect: usize = 0xC0; //
    }
    // Parent: C_BaseEntity
    // Field count: 36
    pub mod C_BaseModelEntity {
        pub const m_CRenderComponent: usize = 0x560; //
        pub const m_CHitboxComponent: usize = 0x568; //
        pub const m_LastHitGroup: usize = 0x590; //
        pub const m_bInitModelEffects: usize = 0x5B8; //
        pub const m_bIsStaticProp: usize = 0x5B9; //
        pub const m_nLastAddDecal: usize = 0x5BC; //
        pub const m_nDecalsAdded: usize = 0x5C0; //
        pub const m_iOldHealth: usize = 0x5C4; //
        pub const m_nRenderMode: usize = 0x5C8; //
        pub const m_nRenderFX: usize = 0x5C9; //
        pub const m_szAddModifier: usize = 0x5D0; //
        pub const m_bAllowFadeInView: usize = 0x5D8; //
        pub const m_bHasCollision: usize = 0x5F8; //
        pub const m_vSupport: usize = 0x5FC; //
        pub const m_clrRender: usize = 0x608; //
        pub const m_vecRenderAttributes: usize = 0x610; //
        pub const m_bRenderToCubemaps: usize = 0x690; //
        pub const m_bNoInterpolate: usize = 0x691; //
        pub const m_Collision: usize = 0x698; //
        pub const m_Glow: usize = 0x748; //
        pub const m_flGlowBackfaceMult: usize = 0x7A0; //
        pub const m_fadeMinDist: usize = 0x7A4; //
        pub const m_fadeMaxDist: usize = 0x7A8; //
        pub const m_flFadeScale: usize = 0x7AC; //
        pub const m_flShadowStrength: usize = 0x7B0; //
        pub const m_nObjectCulling: usize = 0x7B4; //
        pub const m_nAddDecal: usize = 0x7B8; //
        pub const m_vDecalPosition: usize = 0x7BC; //
        pub const m_vDecalForwardAxis: usize = 0x7C8; //
        pub const m_flDecalHealBloodRate: usize = 0x7D4; //
        pub const m_flDecalHealHeightRate: usize = 0x7D8; //
        pub const m_ConfigEntitiesToPropagateMaterialDecalsTo: usize = 0x7E0; //
        pub const m_vecViewOffset: usize = 0x7F8; //
        pub const m_pClientAlphaProperty: usize = 0x828; //
        pub const m_ClientOverrideTint: usize = 0x830; //
        pub const m_bUseClientOverrideTint: usize = 0x834; //
    }
    // Parent: C_BaseEntity
    // Field count: 1
    pub mod C_MiniMapMarker {
        pub const m_eType: usize = 0x560; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierRapidFireChannelVData {
        pub const m_flAirDrag: usize = 0x638; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_BulletFlurryVData {
        pub const m_ChannelParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BulletFlurryModifier: usize = 0x1670; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CGameModifier_SetModelScale {
        pub const m_flOldModelScale: usize = 0xC0; //
    }
    // Parent: C_BaseClientUIEntity
    // Field count: 13
    pub mod C_PointClientUIHUD {
        pub const m_bCheckCSSClasses: usize = 0x890; //
        pub const m_bIgnoreInput: usize = 0xA10; //
        pub const m_flWidth: usize = 0xA14; //
        pub const m_flHeight: usize = 0xA18; //
        pub const m_flDPI: usize = 0xA1C; //
        pub const m_flInteractDistance: usize = 0xA20; //
        pub const m_flDepthOffset: usize = 0xA24; //
        pub const m_unOwnerContext: usize = 0xA28; //
        pub const m_unHorizontalAlign: usize = 0xA2C; //
        pub const m_unVerticalAlign: usize = 0xA30; //
        pub const m_unOrientation: usize = 0xA34; //
        pub const m_bAllowInteractionFromAllSceneWorlds: usize = 0xA38; //
        pub const m_vecCSSClasses: usize = 0xA40; // C_NetworkUtlVectorBase<CUtlSymbolLarge>
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CModifier_Synth_Blitz_TechAmp_VData {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_WreckingBall_Debuff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Puddle {
    }
    // Parent: CitadelItemVData
    // Field count: 4
    pub mod CCitadel_Item_TechDamagePulseVData {
        pub const m_PulseParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetParticle: usize = 0x16B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strPulseTickSound: usize = 0x1798; //
        pub const m_iMaxTargets: usize = 0x17A8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_BulletArmorReduction {
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityProperty_TechRange {
    }
    // Parent: C_BaseModelEntity
    // Field count: 2
    pub mod C_SpotlightEnd {
        pub const m_flLightScale: usize = 0x858; //
        pub const m_Radius: usize = 0x85C; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Bolo_Leech {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 9
    pub mod CCitadel_Ability_BulletFlurry {
        pub const m_flFlurryEndTime: usize = 0xCA0; //
        pub const m_flNextAttackTime: usize = 0xCB8; //
        pub const m_vecShootTargets: usize = 0xDD8; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_nNumPlayersKilled: usize = 0xDF0; //
        pub const m_nShootIndex: usize = 0xDF4; //
        pub const m_nShootIndexNPC: usize = 0xDF8; //
        pub const m_nBurstShots: usize = 0xDFC; //
        pub const m_nSatVolumeIndex: usize = 0xE00; //
        pub const m_bHasCameraOverride: usize = 0xE04; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 6
    pub mod C_NPC_Boss_Tier2 {
        pub const m_iLane: usize = 0x1428; //
        pub const m_flFadeOutStart: usize = 0x142C; //
        pub const m_flFadeOutEnd: usize = 0x1430; //
        pub const m_hTargetedEnemy: usize = 0x1434; //
        pub const m_vecElectricBeamLookTarget: usize = 0x1438; //
        pub const m_nElectricBeamCasts: usize = 0x1450; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadelModifierAirLiftExplodeAuraVData {
        pub const m_empWaveParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_BaseEntity
    // Field count: 26
    pub mod C_PointCamera {
        pub const m_FOV: usize = 0x560; //
        pub const m_Resolution: usize = 0x564; //
        pub const m_bFogEnable: usize = 0x568; //
        pub const m_FogColor: usize = 0x569; //
        pub const m_flFogStart: usize = 0x570; //
        pub const m_flFogEnd: usize = 0x574; //
        pub const m_flFogMaxDensity: usize = 0x578; //
        pub const m_bActive: usize = 0x57C; //
        pub const m_bUseScreenAspectRatio: usize = 0x57D; //
        pub const m_flAspectRatio: usize = 0x580; //
        pub const m_bNoSky: usize = 0x584; //
        pub const m_fBrightness: usize = 0x588; //
        pub const m_flZFar: usize = 0x58C; //
        pub const m_flZNear: usize = 0x590; //
        pub const m_bCanHLTVUse: usize = 0x594; //
        pub const m_bAlignWithParent: usize = 0x595; //
        pub const m_bDofEnabled: usize = 0x596; //
        pub const m_flDofNearBlurry: usize = 0x598; //
        pub const m_flDofNearCrisp: usize = 0x59C; //
        pub const m_flDofFarCrisp: usize = 0x5A0; //
        pub const m_flDofFarBlurry: usize = 0x5A4; //
        pub const m_flDofTiltToGround: usize = 0x5A8; //
        pub const m_TargetFOV: usize = 0x5AC; //
        pub const m_DegreesPerSecond: usize = 0x5B0; //
        pub const m_bIsOn: usize = 0x5B4; //
        pub const m_pNext: usize = 0x5B8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Rutger_Pulse {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadelModifierAerialAssaultWatcherVData {
        pub const m_AssaultModifier: usize = 0x638; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_Item_ColdFrontVData {
        pub const m_AOEModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PreventHealing {
    }
    // Parent: C_LightEntity
    // Field count: 0
    pub mod C_LightSpotEntity {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_WeaponUpgrade_HeadshotBooster {
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_Crescendo_PostAOE_VData {
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProcVData
    // Field count: 6
    pub mod CCitadel_Modifier_SalvoBulletVData {
        pub const m_DebuffModifier: usize = 0x768; //
        pub const m_ExplosionParticle: usize = 0x778; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionVictimParticle: usize = 0x858; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SalvoWeaponParticle: usize = 0x938; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShotVictimSound: usize = 0xA18; //
        pub const m_ShotConfirmationSound: usize = 0xA28; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CModifierRapidFireAirJuggleVData {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityCrowdControlVData {
        pub const m_CastParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SlowModifier: usize = 0x1670; //
        pub const m_DebuffModifier: usize = 0x1680; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 19
    pub mod CCitadel_Ability_ProximityRitual_VData {
        pub const m_PredatoryStatueModel: usize = 0x1590; //
        pub const m_CatReappearParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CatDisappearParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CatEyesParticle: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CatSummonParticle: usize = 0x1910; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CatRecallParticle: usize = 0x19F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RecallLineParticle: usize = 0x1AD0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strRecallSound: usize = 0x1BB0; //
        pub const m_strKilledSound: usize = 0x1BC0; //
        pub const m_PredatoryStatueModifier: usize = 0x1BD0; //
        pub const m_RecentDamageModifier: usize = 0x1BE0; //
        pub const m_flHeavyMeleeDmg: usize = 0x1BF0; //
        pub const m_flLightMeleeDmg: usize = 0x1BF4; //
        pub const m_flAbilityDamageScale: usize = 0x1BF8; //
        pub const m_flNPCDamageScale: usize = 0x1BFC; //
        pub const m_flCastDelayMin: usize = 0x1C00; //
        pub const m_flCastDelayMax: usize = 0x1C04; //
        pub const m_flCastDelayMaxDist: usize = 0x1C08; //
        pub const m_flPostCastCooldown: usize = 0x1C0C; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_PowerJump {
        pub const m_nTargetingParticleIndex: usize = 0xCA4; //
        pub const m_bAirRaiding: usize = 0xCA8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CModifierTier3BossLaserBeamVData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 12
    pub mod CCitadel_Ability_Climb_Rope {
        pub const m_vTop: usize = 0xCA0; //
        pub const m_vBottom: usize = 0xCD0; //
        pub const m_flActivatePressTime: usize = 0xD00; //
        pub const m_flDisconnectTime: usize = 0xD04; //
        pub const m_flClimbStartTime: usize = 0xD08; //
        pub const m_vLastPos: usize = 0xD0C; //
        pub const m_bRequestStopClimbing: usize = 0xD20; //
        pub const m_bRequestJumpToRoof: usize = 0xD21; //
        pub const m_flLastMoveTime: usize = 0xD24; //
        pub const m_flMoveDownStartTime: usize = 0xD28; //
        pub const m_eClimbState: usize = 0xD2C; //
        pub const m_ClimbCount: usize = 0xD34; //
    }
    // Parent: C_Team
    // Field count: 10
    pub mod C_CitadelTeam {
        pub const m_flBaseObjectiveHealth: usize = 0x618; //
        pub const m_vecBaseLocationX: usize = 0x61C; //
        pub const m_vecBaseLocationY: usize = 0x620; //
        pub const m_bHasValidBaseLocation: usize = 0x624; //
        pub const m_hPayload: usize = 0x628; //
        pub const m_nBossesAlive: usize = 0x62C; //
        pub const m_nBossesMax: usize = 0x630; //
        pub const m_nFlexSlotsUnlocked: usize = 0x634; //
        pub const m_nBaseGuardianLanesCleared: usize = 0x638; //
        pub const m_vecFOWEntities: usize = 0x640; // C_UtlVectorEmbeddedNetworkVar<STeamFOWEntity>
    }
    // Parent: C_BaseEntity
    // Field count: 1
    pub mod C_EnvWind {
        pub const m_EnvWindShared: usize = 0x560; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CAbility_Operative_UmbrellaManeuver_Attack_VData {
        pub const m_ChannelParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChannelStartParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShootParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeParticle: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LaunchParticle: usize = 0x1910; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x19F0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 5
    pub mod CCitadel_Ability_Mirage_Teleport {
        pub const m_hDummyForCamera: usize = 0xCA8; //
        pub const m_hTarget: usize = 0xCAC; //
        pub const m_vCastStartPosition: usize = 0xCB0; //
        pub const m_vTargetPosition: usize = 0xCBC; //
        pub const m_vTargetAngles: usize = 0xCC8; //
    }
    // Parent: CCitadelModifier
    // Field count: 5
    pub mod CCitadel_Modifier_Warden_RiotProtocol {
        pub const m_mapEntToTimeHit: usize = 0xC0; //
        pub const m_nNumPlayersAffected: usize = 0xE8; //
        pub const m_nNumPlayersKilled: usize = 0xEC; //
        pub const m_playerAngles: usize = 0xF0; //
        pub const m_ConeParticle: usize = 0xFC; //
    }
    // Parent: CAbilityMeleeVData
    // Field count: 14
    pub mod CAbilityHoldMelee_VData {
        pub const m_mapAttacks: usize = 0x15B8; //
        pub const m_flNextAttackOnParry: usize = 0x15E0; //
        pub const m_flParryWindow: usize = 0x15E4; //
        pub const m_flParryStunTime: usize = 0x15E8; //
        pub const m_flParryCooldown: usize = 0x15EC; //
        pub const m_AirMeleeUpScale: usize = 0x15F0; //
        pub const m_HoldBeginEffect: usize = 0x1600; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SuccessfulParryParticle: usize = 0x16E0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ParryActivateParticle: usize = 0x17C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_cameraSequenceHoldStart: usize = 0x18A0; //
        pub const m_cameraSequenceHitImpact: usize = 0x1928; //
        pub const m_strHoldBegin: usize = 0x19B0; //
        pub const m_strSuccessfulParrySound: usize = 0x19C0; //
        pub const m_ParryVictimModifier: usize = 0x19D0; //
    }
    // Parent: CCitadel_Modifier_Intrinsic_BaseVData
    // Field count: 1
    pub mod CCitadel_Modifier_ReinforcingCasingsVData {
        pub const m_BuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SlowImmunity {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_VisibleDuration {
    }
    // Parent: CAI_CitadelNPCVData
    // Field count: 40
    pub mod CAI_NPC_TrooperVData {
        pub const m_TrooperType: usize = 0x10E8; //
        pub const m_flTrooperDamageResistPct: usize = 0x10EC; //
        pub const m_flT1BossDamageResistPct: usize = 0x10F0; //
        pub const m_flT2BossDamageResistPct: usize = 0x10F4; //
        pub const m_flBarrackGuardianDamageResistPct: usize = 0x10F8; //
        pub const m_flNearDeathDuration: usize = 0x10FC; //
        pub const m_flFlySpeed: usize = 0x1100; //
        pub const m_flFlyHeight: usize = 0x1104; //
        pub const m_flMeleeDamage: usize = 0x1108; //
        pub const m_flMeleeDuration: usize = 0x110C; //
        pub const m_flMeleeChargeRange: usize = 0x1110; //
        pub const m_flAttackT1BossMaxRange: usize = 0x1114; //
        pub const m_flAttackTrooperMaxRange: usize = 0x1118; //
        pub const m_flShieldDamageResistPct: usize = 0x111C; //
        pub const m_flHealthBarOffsetDucking: usize = 0x1120; //
        pub const m_flTrooperDPS: usize = 0x1124; //
        pub const m_flPlayerDPS: usize = 0x1128; //
        pub const m_flT1BossDPS: usize = 0x112C; //
        pub const m_flT1BossDPSBaseResist: usize = 0x1130; //
        pub const m_flT1BossDPSMaxResist: usize = 0x1134; //
        pub const m_flT1BossDPSMaxResistTimeInSeconds: usize = 0x1138; //
        pub const m_flT2BossDPS: usize = 0x113C; //
        pub const m_flT2BossDPSBaseResist: usize = 0x1140; //
        pub const m_flT2BossDPSMaxResist: usize = 0x1144; //
        pub const m_flT2BossDPSMaxResistTimeInSeconds: usize = 0x1148; //
        pub const m_flT3BossDPS: usize = 0x114C; //
        pub const m_flBarrackBossDPS: usize = 0x1150; //
        pub const m_flGeneratorBossDPS: usize = 0x1154; //
        pub const m_BossAttackParticle: usize = 0x1158; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LastHitParticle: usize = 0x1238; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetingLaserParticle: usize = 0x1318; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetingEyeFlashParticle: usize = 0x13F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sZiplineContainerBreakFromDamageParticle: usize = 0x14D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sZiplineContainerBreakFromLandingParticle: usize = 0x15B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_MedicHealActiveParticle: usize = 0x1698; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sPlayerLastHitSound: usize = 0x1778; //
        pub const m_sCelebrationSound: usize = 0x1788; //
        pub const m_sZiplineContainerBreakSound: usize = 0x1798; //
        pub const m_NearDeathModifier: usize = 0x17A8; //
        pub const m_TrooperBossInvulnModifier: usize = 0x17B8; //
    }
    // Parent: CCitadel_Modifier_ScalingPowerUp
    // Field count: 1
    pub mod CCitadel_Modifier_PowerUp_Movement {
        pub const m_bFilled: usize = 0xC0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CAbility_Fathom_ReefdwellerHarpoon_VData {
        pub const m_DetachBuff: usize = 0x1590; // CEmbeddedSubclass<CCitadel_Modifier_ReefdwellerHarpoon_DetachBuff>
        pub const m_strSwapStarted: usize = 0x15A0; //
        pub const m_cameraSequenceFlying: usize = 0x15B0; //
        pub const m_flAirSpeedMax: usize = 0x1638; //
        pub const m_flFallSpeedMax: usize = 0x163C; //
        pub const m_flAirDrag: usize = 0x1640; //
        pub const m_flInitialSlowSpeed: usize = 0x1644; //
        pub const m_flInitialSpeedBias: usize = 0x1648; //
        pub const m_flMaxSurfacePitch: usize = 0x164C; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_SandPhantom_WhirlwindEvasion {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Gunslinger_SalvoVData {
        pub const m_BulletWarningParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ProcWatcherModifier: usize = 0x1670; //
        pub const m_VictimWarningModifier: usize = 0x1680; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_LifeDrain {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_StaticChargeVData {
        pub const m_ExplodeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZapParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CItemRefresherVData {
        pub const m_RefreshParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Hero_Testing_Damage_AuraDebuff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PowerUp {
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod C_GameRulesProxy {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ChargingGun {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CItemSmokeBombPreCastModifierVData {
        pub const m_SmokeAreaParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CasterParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_TechBurst_Proc {
    }
    // Parent: C_BaseFlex
    // Field count: 6
    pub mod C_BaseCombatCharacter {
        pub const m_hMyWearables: usize = 0xC90; //
        pub const m_leftFootAttachment: usize = 0xCA8; //
        pub const m_rightFootAttachment: usize = 0xCA9; //
        pub const m_nWaterWakeMode: usize = 0xCAC; // C_BaseCombatCharacter::WaterWakeMode_t
        pub const m_flWaterWorldZ: usize = 0xCB0; //
        pub const m_flWaterNextTraceTime: usize = 0xCB4; //
    }
    // Parent: CCitadelBaseTriggerAbility
    // Field count: 0
    pub mod CCitadel_Ability_WreckingBallThrow {
    }
    // Parent: CEnvSoundscape
    // Field count: 0
    pub mod CEnvSoundscapeTriggerable {
    }
    // Parent: C_SoundEventEntity
    // Field count: 1
    pub mod C_SoundEventSphereEntity {
        pub const m_flRadius: usize = 0x620; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Cadence_Crescendo {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SpilledBloodThinker {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_Upgrade_StabilizingTripodVData {
        pub const m_SelfDebuffModifier: usize = 0x15D8; //
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 2
    pub mod CCitadelBulletTimeWarpVData {
        pub const m_TimeWallHitParticle: usize = 0x28; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TimeWallHitTimerParticle: usize = 0x108; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_DynamicProp
    // Field count: 0
    pub mod C_DynamicPropAlias_cable_dynamic {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifierItemPickupAuraTargetVData {
        pub const m_PickupTimer: usize = 0x638; //
        pub const m_PickupTimerModifier: usize = 0x640; //
    }
    // Parent: C_PointEntity
    // Field count: 12
    pub mod C_SceneEntity {
        pub const m_bIsPlayingBack: usize = 0x568; //
        pub const m_bPaused: usize = 0x569; //
        pub const m_bMultiplayer: usize = 0x56A; //
        pub const m_bAutogenerated: usize = 0x56B; //
        pub const m_flForceClientTime: usize = 0x56C; //
        pub const m_nSceneStringIndex: usize = 0x570; //
        pub const m_bClientOnly: usize = 0x572; //
        pub const m_hOwner: usize = 0x574; //
        pub const m_hActorList: usize = 0x578; //
        pub const m_bWasPlaying: usize = 0x590; //
        pub const m_QueuedEvents: usize = 0x5A0; //
        pub const m_flCurrentTime: usize = 0x5B8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Trapper_Fear {
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CCitadel_Ability_Tokamak_DyingStarVData {
        pub const m_ExplosionParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FlameAuraParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strInFlightAnimGraphParam: usize = 0x1750; //
        pub const m_strExplodeSound: usize = 0x1758; //
        pub const m_InFlightModifier: usize = 0x1768; //
    }
    // Parent: CCitadel_Ability_PrimaryWeaponVData
    // Field count: 7
    pub mod CCitadel_Ability_PrimaryWeapon_BebopVData {
        pub const m_strWindupSound: usize = 0x15E0; //
        pub const m_strBeamStartSound: usize = 0x15F0; //
        pub const m_strBeamLoopSound1: usize = 0x1600; //
        pub const m_strBeamLoopSound2: usize = 0x1610; //
        pub const m_strBeamStopSound: usize = 0x1620; //
        pub const m_szWeaponBeamParticle: usize = 0x1630; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flWindupRepeatCycle: usize = 0x1710; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_ProjectMindVData {
        pub const m_TeleportStartParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportEndParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportTrailParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportModelParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShieldModifier: usize = 0x9B8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CModifier_SiphonBullets {
    }
    // Parent: CCitadel_Item_BubbleVData
    // Field count: 1
    pub mod CCitadel_Item_Stasis_BombVData {
        pub const m_AuraModifier: usize = 0x16D8; //
    }
    // Parent: CPlayerPawnComponent
    // Field count: 21
    pub mod CPlayer_CameraServices {
        pub const m_vecPunchAngle: usize = 0x40; //
        pub const m_vecPunchAngleVel: usize = 0x58; //
        pub const m_nPunchAngleJoltTickClientSide: usize = 0x70; //
        pub const m_nPunchAngleJoltTick: usize = 0x74; //
        pub const m_PlayerFog: usize = 0x78; //
        pub const m_hColorCorrectionCtrl: usize = 0xB8; //
        pub const m_hViewEntity: usize = 0xBC; //
        pub const m_hTonemapController: usize = 0xC0; //
        pub const m_audio: usize = 0xC8; //
        pub const m_PostProcessingVolumes: usize = 0x140; //
        pub const m_flOldPlayerZ: usize = 0x158; //
        pub const m_flOldPlayerViewOffsetZ: usize = 0x15C; //
        pub const m_CurrentFog: usize = 0x160; //
        pub const m_hOldFogController: usize = 0x1C8; //
        pub const m_bOverrideFogColor: usize = 0x1CC; //
        pub const m_OverrideFogColor: usize = 0x1D1; //
        pub const m_bOverrideFogStartEnd: usize = 0x1E5; //
        pub const m_fOverrideFogStart: usize = 0x1EC; //
        pub const m_fOverrideFogEnd: usize = 0x200; //
        pub const m_hActivePostProcessingVolume: usize = 0x214; // CHandle<C_PostProcessingVolume>
        pub const m_angDemoViewAngles: usize = 0x218; //
    }
    // Parent: C_BaseTrigger
    // Field count: 0
    pub mod C_TriggerMultiple {
    }
    // Parent: C_FuncBrush
    // Field count: 8
    pub mod C_FuncMonitor {
        pub const m_targetCamera: usize = 0x858; //
        pub const m_nResolutionEnum: usize = 0x860; //
        pub const m_bRenderShadows: usize = 0x864; //
        pub const m_bUseUniqueColorTarget: usize = 0x865; //
        pub const m_brushModelName: usize = 0x868; //
        pub const m_hTargetCamera: usize = 0x870; //
        pub const m_bEnabled: usize = 0x874; //
        pub const m_bDraw3DSkybox: usize = 0x875; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PowerGenerator {
    }
    // Parent: CCitadel_Modifier_Burning
    // Field count: 0
    pub mod CCitadel_Modifier_SpreadingFire_DOT {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Chrono_PulseGrenade_VData {
        pub const m_PulseAreaModifier: usize = 0x1590; //
        pub const m_strHitSound: usize = 0x15A0; //
        pub const m_strDebuffStatName: usize = 0x15B0; //
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 0
    pub mod CCitadel_Ability_Tier2Boss_Stomp {
    }
    // Parent: None
    // Field count: 1
    pub mod C_RopeKeyframe__CPhysicsDelegate {
        pub const m_pKeyframe: usize = 0x8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CModifierVData_BaseAura {
        pub const m_eAuraShapeType: usize = 0x638; //
        pub const m_flAuraRadius: usize = 0x63C; //
        pub const m_flAuraEntityBoundsScale: usize = 0x64C; //
        pub const m_nAmbientParticleRadiusControlPoint: usize = 0x65C; //
        pub const m_modifierProvidedByAura: usize = 0x660; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CModifier_Mirage_FireBeetles_StatStealDebuff_VData {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Synth_Pulse_Escape_VData {
        pub const m_SatchelParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CCitadelAbilityTangoTetherVData {
        pub const m_TetherModifier: usize = 0x1590; //
        pub const m_GrappleTargetModifier: usize = 0x15A0; //
        pub const m_BulletGrappleTracerParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyGrappleParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDamageTarget: usize = 0x1770; //
        pub const m_strGrappleHitTarget: usize = 0x1780; //
        pub const m_strGrappleHitWorld: usize = 0x1790; //
        pub const m_strGrappleHitNothing: usize = 0x17A0; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_WeaponUpgrade_BansheeSlugs_VData {
        pub const m_DebuffModifier: usize = 0x15D8; //
        pub const m_DebuffPerCasterModifier: usize = 0x15E8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_TechDefenderShreddersProcVData {
        pub const m_TechDebuffModifier: usize = 0x668; //
        pub const m_ImpactParticle: usize = 0x678; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_DivinersKevlarBuff_VData {
        pub const m_KevlarChannelParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CItem_Infuser_VData {
        pub const m_BuffModifier: usize = 0x15D8; //
        pub const m_CastParticle: usize = 0x15E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseShield
    // Field count: 0
    pub mod CCitadel_Modifier_RegeneratingTechShield {
    }
    // Parent: CEntityComponent
    // Field count: 2
    pub mod CBodyComponent {
        pub const m_pSceneNode: usize = 0x8; //
        pub const __m_pChainEntity: usize = 0x48; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Upgrade_OverdriveClip {
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod CCitadelModelEntity {
    }
    // Parent: C_SoundAreaEntityBase
    // Field count: 1
    pub mod C_SoundAreaEntitySphere {
        pub const m_flRadius: usize = 0x588; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_VoidSphere_Buff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_StunnedVData {
        pub const m_StunnedParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_EscalatingExposure {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ItemPickupAuraTarget {
    }
    // Parent: C_BaseModelEntity
    // Field count: 8
    pub mod C_LightGlow {
        pub const m_nHorizontalSize: usize = 0x858; //
        pub const m_nVerticalSize: usize = 0x85C; //
        pub const m_nMinDist: usize = 0x860; //
        pub const m_nMaxDist: usize = 0x864; //
        pub const m_nOuterMaxDist: usize = 0x868; //
        pub const m_flGlowProxySize: usize = 0x86C; //
        pub const m_flHDRColorScale: usize = 0x870; //
        pub const m_GlowOverlay: usize = 0x878; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CAbility_Operative_UmbrellaManeuver_Attack {
        pub const m_ChannelParticle: usize = 0xCA0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Wrecker_Salvage {
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CAbilityCrackshotVData {
        pub const m_ExplosionParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionVictimParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ReadyParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffModifier: usize = 0x1830; //
        pub const m_CrackshotImmuneModifier: usize = 0x1840; //
        pub const m_HeadShotVictimSound: usize = 0x1850; //
        pub const m_HeadShotConfirmationSound: usize = 0x1860; //
        pub const m_ReadySound: usize = 0x1870; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Charged_Bomb {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_SlowingTech_ProcVData {
        pub const m_DebuffModifier: usize = 0x668; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 1
    pub mod CCitadel_Modifier_CharmedWraps {
        pub const m_fLastPrimingLightAttackTime: usize = 0x168; //
    }
    // Parent: CitadelItemVData
    // Field count: 9
    pub mod CItemPhantomStrike_VData {
        pub const m_DebuffModifier: usize = 0x15D8; //
        pub const m_PullDownModifier: usize = 0x15E8; //
        pub const m_CasterModifier: usize = 0x15F8; //
        pub const m_strExplodeSound: usize = 0x1608; //
        pub const m_CastParticle: usize = 0x1618; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x16F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffParticle: usize = 0x17D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flTeleportDistance: usize = 0x18B8; //
        pub const m_flVelocityScale: usize = 0x18BC; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityMantleVData {
        pub const m_vecMantleTypes: usize = 0x1590; //
    }
    // Parent: CCitadel_Modifier_RegeneratingTechShield
    // Field count: 0
    pub mod CCitadel_ModifierTempTechShield {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Basic_RangedArmorBonus {
    }
    // Parent: None
    // Field count: 4
    pub mod CountdownTimer {
        pub const m_duration: usize = 0x8; //
        pub const m_timestamp: usize = 0xC; //
        pub const m_timescale: usize = 0x10; //
        pub const m_nWorldGroupId: usize = 0x14; //
    }
    // Parent: None
    // Field count: 2
    pub mod CGameSceneNodeHandle {
        pub const m_hOwner: usize = 0x8; //
        pub const m_name: usize = 0xC; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_ConditionalCollidable {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_Thumper_1 {
        pub const m_vecAimPos: usize = 0xCA0; //
        pub const m_vecAimNormal: usize = 0xCAC; //
        pub const m_flPushForce: usize = 0xCB8; //
    }
    // Parent: CCitadel_Modifier_Sleep
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_Sleeping {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_SnakeDash {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Running_Decoy {
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_PuddleVData {
        pub const m_PuddleModifier: usize = 0x1590; //
    }
    // Parent: CCitadel_Modifier_IceDome_AuraModifierBase
    // Field count: 0
    pub mod CCitadel_Modifier_IceDomeFriendly {
    }
    // Parent: CCitadel_Modifier_Intrinsic_Base
    // Field count: 3
    pub mod CCitadel_Modifier_NapalmProjectile {
        pub const m_vInitialCastPosition: usize = 0x1D8; //
        pub const m_flProjectileSpeed: usize = 0x1E4; //
        pub const m_iParticleEffect: usize = 0x1E8; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_Item_DPS_Aura_VData {
        pub const m_AOECastParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ActiveModifier: usize = 0x16B8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Upgrade_OverdriveClip {
        pub const m_nBonusMaxClipSize: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Berserker {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_BerserkerDamageStackVData {
        pub const m_BuffStatusParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffStatusParticleEnemy: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CBaseAnimGraph
    // Field count: 14
    pub mod C_ClientRagdoll {
        pub const m_bFadeOut: usize = 0xAF8; //
        pub const m_bImportant: usize = 0xAF9; //
        pub const m_flEffectTime: usize = 0xAFC; //
        pub const m_gibDespawnTime: usize = 0xB00; //
        pub const m_iCurrentFriction: usize = 0xB04; //
        pub const m_iMinFriction: usize = 0xB08; //
        pub const m_iMaxFriction: usize = 0xB0C; //
        pub const m_iFrictionAnimState: usize = 0xB10; //
        pub const m_bReleaseRagdoll: usize = 0xB14; //
        pub const m_iEyeAttachment: usize = 0xB15; //
        pub const m_bFadingOut: usize = 0xB16; //
        pub const m_flScaleEnd: usize = 0xB18; //
        pub const m_flScaleTimeStart: usize = 0xB40; //
        pub const m_flScaleTimeEnd: usize = 0xB68; //
    }
    // Parent: CCitadel_Item_TrackingProjectileApplyModifier
    // Field count: 0
    pub mod CCitadel_Item_Containment {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Magician_Escape {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_IceDome {
        pub const m_flDomeStartTime: usize = 0xCD8; //
        pub const m_flDomeEndTime: usize = 0xCDC; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_DetentionAmmo {
    }
    // Parent: CGameSceneNode
    // Field count: 8
    pub mod CSkeletonInstance {
        pub const m_modelState: usize = 0x170; //
        pub const m_bIsAnimationEnabled: usize = 0x3F0; //
        pub const m_bUseParentRenderBounds: usize = 0x3F1; //
        pub const m_bDisableSolidCollisionsForHierarchy: usize = 0x3F2; //
        pub const m_bDirtyMotionType: usize = 0x0; //
        pub const m_bIsGeneratingLatchedParentSpaceState: usize = 0x0; //
        pub const m_materialGroup: usize = 0x3F4; //
        pub const m_nHitboxSet: usize = 0x3F8; //
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 0
    pub mod C_Citadel_RestorativeGooCube {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_WeaponUpgrade_Ricochet {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 3
    pub mod CCitadel_SpiderAnimatingVData {
        pub const m_sModelName: usize = 0x28; //
        pub const m_flModelScale: usize = 0x108; //
        pub const m_cGlowColor: usize = 0x10C; //
    }
    // Parent: CCitadel_Modifier_Invis
    // Field count: 2
    pub mod CCitadel_Modifier_LurkersAmbush_Invis {
        pub const m_mapStartLookTime: usize = 0x268; //
        pub const m_flStartSpotted: usize = 0x290; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityKobunVData {
        pub const m_vSummonFollowOffset: usize = 0x1590; //
        pub const m_CloneModifier: usize = 0x15A0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Tengu_UrnVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AuraModifier: usize = 0x1670; //
        pub const m_ExplodeSound: usize = 0x1680; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_ThrowSandVData {
        pub const m_SandDebuff: usize = 0x1590; //
        pub const m_SilenceDebuff: usize = 0x15A0; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_Item_RescueBeamVData {
        pub const m_DispelAndHealModifier: usize = 0x15D8; //
        pub const m_PullModifier: usize = 0x15E8; //
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 2
    pub mod C_CitadelPayload {
        pub const m_flProgress: usize = 0xB00; //
        pub const m_nNumPushers: usize = 0xB04; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Fathom_Breach {
        pub const m_nRollFXIndex: usize = 0xCA0; //
        pub const m_bInFlight: usize = 0xCA4; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_RapidFire_AirJuggle {
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 1
    pub mod CCitadel_Modifier_Pillar {
        pub const flAccumulatedDamage: usize = 0xC8; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CItem_FleetfootBoots_VData {
        pub const m_FleetfootBootsModifier: usize = 0x15D8; //
        pub const m_FleetfootBootsBonusClipModifier: usize = 0x15E8; //
    }
    // Parent: CCitadel_Item_TrackingProjectileApplyModifier
    // Field count: 0
    pub mod CItem_WitheringWhip {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Haze_StackingDamage {
        pub const m_nTotalProcs: usize = 0x168; //
    }
    // Parent: CitadelItemVData
    // Field count: 3
    pub mod CCitadel_Item_ModDisruptorVData {
        pub const m_DetonateParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DisruptModifier: usize = 0x16B8; //
        pub const m_flWaveSpeed: usize = 0x16C8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_UnstoppableVData {
        pub const m_ShieldParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PlayerShieldParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_PreMatchWait {
        pub const m_vSpawnPoint: usize = 0xC0; //
    }
    // Parent: CBodyComponentSkeletonInstance
    // Field count: 1
    pub mod CBodyComponentBaseAnimGraph {
        pub const m_animationController: usize = 0x510; //
    }
    // Parent: CBodyComponent
    // Field count: 1
    pub mod CBodyComponentPoint {
        pub const m_sceneNode: usize = 0x80; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Ability_Shield {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_Savior {
    }
    // Parent: C_SoundEventEntity
    // Field count: 0
    pub mod C_SoundEventEntityAlias_snd_event_point {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Operative_UmbrellaManeuver_Watcher {
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CCitadel_Ability_SnakeDashVData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Intimidate {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_ProximityRitual {
        pub const m_eState: usize = 0xCA0; //
        pub const m_hStatue: usize = 0xCA4; //
        pub const m_vLaunchPosition: usize = 0xCB0; //
        pub const m_qLaunchAngle: usize = 0xCBC; //
    }
    // Parent: CCitadel_Modifier_ChainLightning
    // Field count: 0
    pub mod CCitadel_Modifier_Galvanic_Storm {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_GalvanicStormTechShieldVData {
        pub const m_BuffModifier: usize = 0x638; //
        pub const m_ExplodeParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flExplodeSpeed: usize = 0x728; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PullDownToGround {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 26
    pub mod CModifierVData {
        pub const m_flDuration: usize = 0x28; //
        pub const m_bKeepMaximumDurationOnRefresh: usize = 0x38; //
        pub const m_strParticleEffect: usize = 0x40; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strParticleEffectConfig: usize = 0x120; //
        pub const m_strParticleStatusEffect: usize = 0x128; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strParticleStatusEffectConfig: usize = 0x208; //
        pub const m_strScreenParticleEffect: usize = 0x210; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strScreenParticleEffectConfig: usize = 0x2F0; //
        pub const m_nStatusEffectPriority: usize = 0x2F8; //
        pub const m_vecRenderAttributes: usize = 0x300; //
        pub const m_sStartSound: usize = 0x318; //
        pub const m_sAmbientLoopingSound: usize = 0x328; //
        pub const m_nAmbientLoopingSoundRecipients: usize = 0x338; //
        pub const m_sEndSound: usize = 0x340; //
        pub const m_nEnabledStateMask: usize = 0x350; //
        pub const m_nDisabledStateMask: usize = 0x36C; //
        pub const m_nAttributes: usize = 0x388; //
        pub const m_vecScriptValues: usize = 0x390; //
        pub const m_vecScriptEventHandlers: usize = 0x3A8; //
        pub const m_nDisableGroupsMask: usize = 0x3C0; //
        pub const m_bPrivateAccess: usize = 0x3C4; //
        pub const m_bIsHidden: usize = 0x3C5; //
        pub const m_eHiddenType: usize = 0x3C8; //
        pub const m_sLocalizationName: usize = 0x3D0; //
        pub const m_eDebuffType: usize = 0x3D8; //
        pub const m_bAutomaticallyDecayStacks: usize = 0x3DC; //
    }
    // Parent: C_NPC_FlyingDrone
    // Field count: 0
    pub mod C_NPC_SurveillanceDrone {
    }
    // Parent: C_DynamicProp
    // Field count: 0
    pub mod CCitadel_ShopProp {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Yamato_InfinitySlash_BuffTimer {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_LockDown {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Uppercutted {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 5
    pub mod CCitadel_Ability_Bounce_Pad {
        pub const m_vForward: usize = 0xCA0; //
        pub const m_bShouldDeploy: usize = 0xCAC; //
        pub const m_bAnglesSet: usize = 0xCAD; //
        pub const m_bCanCancel: usize = 0xCAE; //
        pub const m_angFacing: usize = 0xDC8; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CItemPowerShardVData {
        pub const m_RefreshParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_Tier2Boss_RocketBarrageVData {
        pub const m_ExplosionParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionSound: usize = 0x1670; //
        pub const m_RocketFireSound: usize = 0x1680; //
        pub const m_AuraModifier: usize = 0x1690; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod CNPC_YakuzaGangster {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CAbility_Mirage_SandPhantom {
        pub const m_vecVictimModifiers: usize = 0xCA0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_ThrowSandDebuffVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Upgrade_SpiritSnatch_Debuff {
    }
    // Parent: C_BasePlayerPawn
    // Field count: 0
    pub mod CCitadelPlayerPawnBase {
    }
    // Parent: CBaseAnimGraph
    // Field count: 0
    pub mod C_ItemFlare {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_ReturnFire {
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 7
    pub mod CCitadel_Modifier_Knockdown {
        pub const m_angStunAngles: usize = 0xC8; //
        pub const m_ePreferredKnockdownType: usize = 0xD4; //
        pub const m_bForceTakePreferred: usize = 0xD8; //
        pub const m_flGetUpAnimTime: usize = 0xDC; //
        pub const m_bGetUpCamSeqStarted: usize = 0xE0; //
        pub const m_bOnGroundDuration: usize = 0xE1; //
        pub const m_satIndex: usize = 0xE4; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityLashVData {
        pub const m_LashParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffModifier: usize = 0x1670; //
        pub const m_strVictimCastSound: usize = 0x1680; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_ChargedBombVData {
        pub const m_ChargeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strBeepSound: usize = 0x718; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 3
    pub mod C_Citadel_Projectile_Viscous_GooGrenade {
        pub const m_nBounces: usize = 0x8E0; //
        pub const m_tNextDetonateTime: usize = 0x8E4; //
        pub const m_vecProjectileHitTargets: usize = 0x8E8; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Trapper_Immobilize {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Thumper_EnemyPulled {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_RapidFire {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 2
    pub mod CCitadel_CatAnimatingVData {
        pub const m_sModelName: usize = 0x28; //
        pub const m_cGlowColor: usize = 0x108; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_WreckerGarbageSuck {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_RestorativeGoo {
        pub const m_flSelfCastEndTime: usize = 0xCA0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Idol_Return {
    }
    // Parent: C_RagdollProp
    // Field count: 7
    pub mod C_RagdollPropAttached {
        pub const m_boneIndexAttached: usize = 0xB70; //
        pub const m_ragdollAttachedObjectIndex: usize = 0xB74; //
        pub const m_attachmentPointBoneSpace: usize = 0xB78; //
        pub const m_attachmentPointRagdollSpace: usize = 0xB84; //
        pub const m_vecOffset: usize = 0xB90; //
        pub const m_parentTime: usize = 0xB9C; //
        pub const m_bHasParent: usize = 0xBA0; //
    }
    // Parent: C_BaseToggle
    // Field count: 11
    pub mod C_BaseTrigger {
        pub const m_OnStartTouch: usize = 0x858; //
        pub const m_OnStartTouchAll: usize = 0x880; //
        pub const m_OnEndTouch: usize = 0x8A8; //
        pub const m_OnEndTouchAll: usize = 0x8D0; //
        pub const m_OnTouching: usize = 0x8F8; //
        pub const m_OnTouchingEachEntity: usize = 0x920; //
        pub const m_OnNotTouching: usize = 0x948; //
        pub const m_hTouchingEntities: usize = 0x970; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_iFilterName: usize = 0x988; //
        pub const m_hFilter: usize = 0x990; //
        pub const m_bDisabled: usize = 0x994; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Magician_CopyUltVData {
        pub const m_CopyTetherParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_UltCopiedModifier: usize = 0x1670; //
        pub const m_UltActiveModifier: usize = 0x1680; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityHighAlertVData {
        pub const m_BuffModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_ZiplineBoost {
        pub const m_bIsBoosting: usize = 0xC0; //
    }
    // Parent: None
    // Field count: 3
    pub mod C_EconEntity__AttachedParticleInfo_t {
        pub const m_nAttachedParticleIndex: usize = 0x0; //
        pub const m_customType: usize = 0x4; //
        pub const m_bShouldDestroyImmediately: usize = 0x8; //
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProc
    // Field count: 1
    pub mod CCitadel_Modifier_SalvoBullet {
        pub const m_BuffedShotId: usize = 0x1F8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_BoloVData {
        pub const m_TrapModifier: usize = 0x638; //
        pub const m_ReverseLeechModifier: usize = 0x648; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 6
    pub mod CCitadel_Ability_Chrono_KineticCarbine {
        pub const m_bWantsSlow: usize = 0xCA0; //
        pub const m_flLatchedTimeScaleFracChangeTime: usize = 0xCA4; //
        pub const m_flLatchedTimeScaleFrac: usize = 0xCA8; //
        pub const m_flSpeedBoostEndTime: usize = 0xCAC; //
        pub const m_flShotTimeScaleEndTime: usize = 0xCB0; //
        pub const m_flStoredPowerPct: usize = 0xCB8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_DeathTaxTechAmp {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_Item_BaseProjectileAOEModifierVData {
        pub const m_AOEModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ColossusActive {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_AblativeCoat {
        pub const m_iCurrentResistValue: usize = 0xCB8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_PermanentPickupVData {
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_CopyUltVData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Magician_ShadowClone {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbility_Mirage_SandPhantom_VData {
        pub const m_WhirlwindEvasionModifier: usize = 0x1590; //
        pub const m_SandPhantomModifier: usize = 0x15A0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Cadence_GrandFinale {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_Gunslinger_Salvo {
        pub const m_CastTarget: usize = 0xCA4; //
        pub const m_iCurrentShots: usize = 0xCA8; //
        pub const m_iTotalShots: usize = 0xCAC; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_LockDown_Debuff {
        pub const m_vEscapeTarget: usize = 0x1D8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 5
    pub mod CCitadel_Ability_Bebop_LaserBeam {
        pub const m_bZoomed: usize = 0xDB8; //
        pub const m_bAirCast: usize = 0xDB9; //
        pub const m_vBeamAimPos: usize = 0xDBC; //
        pub const m_angBeamAngles: usize = 0xDC8; //
        pub const m_bNeedsBeamReset: usize = 0xDE0; //
    }
    // Parent: CCitadel_Modifier_Base_Buildup
    // Field count: 1
    pub mod CCitadel_Modifier_IceBeam_Stacking_Slow {
        pub const m_flCurrBuildup: usize = 0x258; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Shield {
        pub const m_hShieldEntity: usize = 0xC0; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_WeaponUpgrade_BurstFireVData {
        pub const m_ActivationSound: usize = 0x15D8; //
        pub const m_BuffModifier: usize = 0x15E8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Basic_HealthRegen {
        pub const m_flHealthRegen: usize = 0xC0; //
    }
    // Parent: None
    // Field count: 21
    pub mod CEffectData {
        pub const m_vOrigin: usize = 0x8; //
        pub const m_vStart: usize = 0x14; //
        pub const m_vNormal: usize = 0x20; //
        pub const m_vAngles: usize = 0x2C; //
        pub const m_hEntity: usize = 0x38; //
        pub const m_hOtherEntity: usize = 0x3C; //
        pub const m_flScale: usize = 0x40; //
        pub const m_flMagnitude: usize = 0x44; //
        pub const m_flRadius: usize = 0x48; //
        pub const m_nSurfaceProp: usize = 0x4C; //
        pub const m_nEffectIndex: usize = 0x50; //
        pub const m_nDamageType: usize = 0x58; //
        pub const m_nPenetrate: usize = 0x5C; //
        pub const m_nMaterial: usize = 0x5E; //
        pub const m_nHitBox: usize = 0x60; //
        pub const m_nColor: usize = 0x62; //
        pub const m_fFlags: usize = 0x63; //
        pub const m_nAttachmentIndex: usize = 0x64; //
        pub const m_nAttachmentName: usize = 0x68; //
        pub const m_iEffectName: usize = 0x6C; //
        pub const m_nExplosionType: usize = 0x6E; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 0
    pub mod CModifierSleepBombAuraVData {
    }
    // Parent: CCitadel_Modifier_Burning
    // Field count: 0
    pub mod CCitadel_Modifier_Tokamak_HeatSinks_DOT {
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilitySummonGangsterVData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ViperVenom {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_MagicCarpet_SummonVData {
        pub const m_SummonParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 0
    pub mod CCitadel_Ability_SuperNeutralShield {
    }
    // Parent: CCitadelPlayerPawnBase
    // Field count: 0
    pub mod C_CitadelObserverPawn {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_SpiderShield {
    }
    // Parent: CitadelAbilityVData
    // Field count: 27
    pub mod CCitadel_Ability_ZipLine_VData {
        pub const m_flZiplineAirDrag: usize = 0x1590; //
        pub const m_flMinButtonHoldTimeToActivate: usize = 0x1594; //
        pub const m_flCrouchDropSpeedFraction: usize = 0x1598; //
        pub const m_flCrouchDropAirDragSuppressDuration: usize = 0x159C; //
        pub const m_flDetachDisallowedTime: usize = 0x15A0; //
        pub const m_flCameraWobbleIntensity: usize = 0x15A4; //
        pub const m_DOFWhileZiplining: usize = 0x15A8; //
        pub const m_ZipLinePreviewParticle: usize = 0x15B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineSpeedParticle: usize = 0x1698; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineTetherParticle: usize = 0x1778; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineTetherAttachParticle: usize = 0x1858; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineTetherStartParticle: usize = 0x1938; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineEnemyKnockdownProtectionParticle: usize = 0x1A18; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineSelfKnockdownProtectionParticle: usize = 0x1AF8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineKnockdownProtectionStatusParticle: usize = 0x1BD8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strZipLineSummonSound: usize = 0x1CB8; //
        pub const m_strZipLineLatchedSound: usize = 0x1CC8; //
        pub const m_strZipLineStartSound: usize = 0x1CD8; //
        pub const m_RidingZipLineModifier: usize = 0x1CE8; //
        pub const m_KnockedOffSlowModifier: usize = 0x1CF8; //
        pub const m_ZipLineIntroModifier: usize = 0x1D08; //
        pub const m_ZipLineKnockdownImmuneModifier: usize = 0x1D18; //
        pub const m_ZipLineSlowModifier: usize = 0x1D28; //
        pub const m_cameraSequenceAwaitingTether: usize = 0x1D38; //
        pub const m_cameraSequenceLatched: usize = 0x1DC0; //
        pub const m_cameraSequenceAttached: usize = 0x1E48; //
        pub const m_cameraSequenceClear: usize = 0x1ED0; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_Colossus_VData {
        pub const m_BuffModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Out_Of_Combat_Health_Regen {
        pub const m_LastDamageTaken: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierGlitchVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod CItemExplosiveBarrel {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_Trapper_SpiderJar {
        pub const m_vLaunchPosition: usize = 0xCA0; //
        pub const m_qLaunchAngle: usize = 0xCAC; //
        pub const m_bHasMadeSpiders: usize = 0xCB8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 7
    pub mod CCitadel_Ability_ViperVenomVData {
        pub const m_BuildUpModifier: usize = 0x1590; //
        pub const m_VenomModifier: usize = 0x15A0; //
        pub const m_CastVenomParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_VenomExplodeParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strVenomWeakExplode: usize = 0x1770; //
        pub const m_strVenomExplode: usize = 0x1780; //
        pub const m_strVenomStrongExplode: usize = 0x1790; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 7
    pub mod CCitadel_Ability_NanoDash {
        pub const m_vStartPosition: usize = 0xCA0; //
        pub const m_vDashDirection: usize = 0xCAC; //
        pub const m_bIsDashing: usize = 0xCB8; //
        pub const m_bStartedInAir: usize = 0xCB9; //
        pub const m_vecHitEnemies: usize = 0xCC0; //
        pub const m_vecLastPosition: usize = 0xCD8; //
        pub const m_flStuckTime: usize = 0xFC0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Stomp {
    }
    // Parent: CCitadelModifierVData
    // Field count: 6
    pub mod CCitadel_Modifier_FissureWallVData {
        pub const m_DebrisParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SpikeParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WallSpawnSound: usize = 0x7F8; //
        pub const m_DebuffModifier: usize = 0x808; //
        pub const m_EnemyVisionModifier: usize = 0x818; //
        pub const m_SlowModifier: usize = 0x828; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Guiding_Arrow_KillCheck {
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_Stunned {
        pub const m_bEnabled: usize = 0xC0; //
        pub const m_bWasEnabled: usize = 0xC1; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ThermalDetonator_Debuff {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_EscalatingExposureProcWatcherVData {
        pub const m_DebuffModifier: usize = 0x668; //
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 0
    pub mod CCitadel_Ability_Tier2Boss_LaserBeam {
    }
    // Parent: C_NPC_Trooper
    // Field count: 0
    pub mod C_NPC_SuperTrooper {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_Item_PowerShard {
        pub const m_hLastSignatureToActivate: usize = 0xCB8; //
    }
    // Parent: CCitadel_Modifier_ScalingPowerUp
    // Field count: 0
    pub mod CCitadel_Modifier_PowerUp_Casting {
    }
    // Parent: C_BaseToggle
    // Field count: 3
    pub mod C_BaseButton {
        pub const m_glowEntity: usize = 0x858; //
        pub const m_usable: usize = 0x85C; //
        pub const m_szDisplayText: usize = 0x860; //
    }
    // Parent: CCitadel_Modifier_StatStealBase
    // Field count: 0
    pub mod CCitadel_Modifier_Mirage_FireScarabs_Watcher {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Tokamak_CrimsonCannon {
        pub const m_bAirCast: usize = 0xCA0; //
        pub const m_bIsZoomed: usize = 0xE68; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_GrandFinaleAOE {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Citadel_Projectile_Wrecker_Teleport {
    }
    // Parent: C_PointEntity
    // Field count: 0
    pub mod CInfoParticleTarget {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_Mirage_FireBeetles {
        pub const m_vLaunchPosition: usize = 0xCA0; //
        pub const m_qLaunchAngle: usize = 0xCAC; //
        pub const m_flCastStartTime: usize = 0xCB8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Gunslinger_SpreadingFire {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_ViperVenomProcWatcherVData {
        pub const m_TracerParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_LightningBall {
        pub const m_hProjectile: usize = 0x168; //
    }
    // Parent: CPlayer_MovementServices
    // Field count: 12
    pub mod CPlayer_MovementServices_Humanoid {
        pub const m_flStepSoundTime: usize = 0x1D8; //
        pub const m_flFallVelocity: usize = 0x1DC; //
        pub const m_bInCrouch: usize = 0x1E0; //
        pub const m_nCrouchState: usize = 0x1E4; //
        pub const m_flCrouchTransitionStartTime: usize = 0x1E8; //
        pub const m_bDucked: usize = 0x1EC; //
        pub const m_bDucking: usize = 0x1ED; //
        pub const m_bInDuckJump: usize = 0x1EE; //
        pub const m_groundNormal: usize = 0x1F0; //
        pub const m_flSurfaceFriction: usize = 0x1FC; //
        pub const m_surfaceProps: usize = 0x200; //
        pub const m_nStepside: usize = 0x210; //
    }
    // Parent: C_NPC_SimpleAnimatingAI
    // Field count: 3
    pub mod C_NPC_ShieldedSentry {
        pub const m_CCitadelAbilityComponent: usize = 0xB00; //
        pub const m_flAttackRange: usize = 0xCA4; //
        pub const m_flAimPitch: usize = 0xCA8; //
    }
    // Parent: CEnvSoundscapeProxy
    // Field count: 0
    pub mod CEnvSoundscapeProxyAlias_snd_soundscape_proxy {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_ColdFront {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_BaseHeldItem {
    }
    // Parent: C_BaseClientUIEntity
    // Field count: 28
    pub mod C_PointClientUIWorldPanel {
        pub const m_bForceRecreateNextUpdate: usize = 0x890; //
        pub const m_bMoveViewToPlayerNextThink: usize = 0x891; //
        pub const m_bCheckCSSClasses: usize = 0x892; //
        pub const m_anchorDeltaTransform: usize = 0x8A0; //
        pub const m_pOffScreenIndicator: usize = 0xA38; //
        pub const m_bIgnoreInput: usize = 0xA60; //
        pub const m_bLit: usize = 0xA61; //
        pub const m_bFollowPlayerAcrossTeleport: usize = 0xA62; //
        pub const m_flWidth: usize = 0xA64; //
        pub const m_flHeight: usize = 0xA68; //
        pub const m_flDPI: usize = 0xA6C; //
        pub const m_flInteractDistance: usize = 0xA70; //
        pub const m_flDepthOffset: usize = 0xA74; //
        pub const m_unOwnerContext: usize = 0xA78; //
        pub const m_unHorizontalAlign: usize = 0xA7C; //
        pub const m_unVerticalAlign: usize = 0xA80; //
        pub const m_unOrientation: usize = 0xA84; //
        pub const m_bAllowInteractionFromAllSceneWorlds: usize = 0xA88; //
        pub const m_vecCSSClasses: usize = 0xA90; // C_NetworkUtlVectorBase<CUtlSymbolLarge>
        pub const m_bOpaque: usize = 0xAA8; //
        pub const m_bNoDepth: usize = 0xAA9; //
        pub const m_bRenderBackface: usize = 0xAAA; //
        pub const m_bUseOffScreenIndicator: usize = 0xAAB; //
        pub const m_bExcludeFromSaveGames: usize = 0xAAC; //
        pub const m_bGrabbable: usize = 0xAAD; //
        pub const m_bOnlyRenderToTexture: usize = 0xAAE; //
        pub const m_bDisableMipGen: usize = 0xAAF; //
        pub const m_nExplicitImageLayout: usize = 0xAB0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Operative_UmbrellaManeuver_Watcher_VData {
        pub const m_AirHangModifier: usize = 0x638; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Trapper_PoisonJar {
        pub const m_vLaunchPosition: usize = 0xCA0; //
        pub const m_qLaunchAngle: usize = 0xCAC; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PoisonJar_Debuff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Fathom_ScaldingSpray_Target {
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CAbility_Synth_PlasmaFlux_VData {
        pub const m_WeaponDamageBonusModifier: usize = 0x1590; //
        pub const m_TeleportTrailParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strCasterLoopingSound: usize = 0x1760; //
        pub const m_strProjectileExpireSound: usize = 0x1770; //
        pub const m_strImpactSound: usize = 0x1780; //
        pub const m_strTimerSound: usize = 0x1790; //
        pub const m_cameraSequenceTeleport: usize = 0x17A0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 10
    pub mod CCitadel_Ability_GooBowlingBall {
        pub const m_nAirJumpsLeft: usize = 0xFB0; //
        pub const m_bIsRolling: usize = 0xFB4; //
        pub const m_hBall: usize = 0xFB8; //
        pub const m_eRollingState: usize = 0xFBC; //
        pub const m_flNextStateTime: usize = 0xFC0; //
        pub const m_flNextWallCheck: usize = 0xFC4; //
        pub const m_flRollStartTime: usize = 0xFC8; //
        pub const m_flWallExitTime: usize = 0xFCC; //
        pub const m_vecWallExitVelocity: usize = 0xFD0; //
        pub const m_nDirectionParticleIndex: usize = 0xFE4; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 7
    pub mod CCitadelModifierAerialAssaultVData {
        pub const m_FireRateModifier: usize = 0x638; //
        pub const m_ExplodeParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TracerParticle: usize = 0x728; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x808; //
        pub const m_flAirDrag: usize = 0x818; //
        pub const m_flAirSpeed: usize = 0x81C; //
        pub const m_flFallSpeed: usize = 0x820; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_SlowingBullets_ProcVData {
        pub const m_DebuffModifier: usize = 0x668; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_ActiveDisarm_SpiritSteal_VData {
        pub const m_SpiritStealParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ClimbRopeSpeed {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadelModifierProjectilePitchingLoopSoundThinkerVData {
        pub const m_speedToPitchRemap: usize = 0x638; //
    }
    // Parent: C_NPC_SimpleAnimatingAI
    // Field count: 2
    pub mod C_NPC_FieldSentry {
        pub const m_flAttackRange: usize = 0xB04; //
        pub const m_flAimPitch: usize = 0xB08; //
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 0
    pub mod C_Citadel_SpiderAnimating {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_ComboBreaker {
    }
    // Parent: CCitadelModifierVData
    // Field count: 10
    pub mod CCitadel_Modifier_Mirage_SandPhantom_Passive_Victim_VData {
        pub const m_SlowModifier: usize = 0x638; //
        pub const m_DebuffStatusPlayerParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffStatusVictimParticle: usize = 0x728; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffStatusNPCParticle: usize = 0x808; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StackDamageParticle: usize = 0x8E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StackReadyParticle: usize = 0x9C8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StackAppliedParticle: usize = 0xAA8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ConsumeMaxStacksSound: usize = 0xB88; //
        pub const m_ConsumeMaxStacksNonHeroSound: usize = 0xB98; //
        pub const m_ApplyStackSound: usize = 0xBA8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Astro_Shotgun_Toggle {
    }
    // Parent: CCitadel_Modifier_Intrinsic_Base
    // Field count: 0
    pub mod CCitadel_Modifier_WeaponEaterStack {
    }
    // Parent: C_SoundOpvarSetPointEntity
    // Field count: 0
    pub mod C_SoundOpvarSetAABBEntity {
    }
    // Parent: None
    // Field count: 4
    pub mod C_GameRules {
        pub const __m_pChainEntity: usize = 0x8; //
        pub const m_nTotalPausedTicks: usize = 0x30; //
        pub const m_nPauseStartTick: usize = 0x34; //
        pub const m_bGamePaused: usize = 0x38; //
    }
    // Parent: C_BasePropDoor
    // Field count: 0
    pub mod C_PropDoorRotating {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod CCitadel_Projectile_WebWall {
    }
    // Parent: C_BaseEntity
    // Field count: 4
    pub mod C_Team {
        pub const m_aPlayerControllers: usize = 0x560; //
        pub const m_aPlayers: usize = 0x578; //
        pub const m_iScore: usize = 0x590; //
        pub const m_szTeamname: usize = 0x594; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_BigBoltVData {
        pub const m_AuraModifier: usize = 0x638; //
        pub const m_ShieldParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flModelScale: usize = 0x728; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 9
    pub mod CCitadel_Ability_ChargedTackle {
        pub const m_bPreparing: usize = 0xE98; //
        pub const m_bTackling: usize = 0xE99; //
        pub const m_flTackleStartTime: usize = 0xE9C; //
        pub const m_flPrepareStartTime: usize = 0xEA0; //
        pub const m_vecTackleDir: usize = 0xEA4; //
        pub const m_vecLastPosition: usize = 0xEB0; //
        pub const m_nStuckFramesCount: usize = 0xEBC; //
        pub const m_vecHitEnemies: usize = 0xEC0; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_nDistancePreview: usize = 0xED8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CCitadelAbilityChargedBombVData {
        pub const m_ChargeBombModifier: usize = 0x1590; //
        pub const m_ExplodeParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExplodeSound: usize = 0x1680; //
        pub const m_flChargeForMaxDamage: usize = 0x1690; //
        pub const m_flMinDamagePercent: usize = 0x1694; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Aerial_Assault_Watcher {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_GlowToTeammates {
    }
    // Parent: CCitadelModifierAura
    // Field count: 3
    pub mod CCitadel_Modifier_AirLift_ExplodeAura {
        pub const m_flStartRadius: usize = 0xF0; //
        pub const m_flEndRadius: usize = 0xF4; //
        pub const m_flSpreadDuration: usize = 0xF8; //
    }
    // Parent: CAI_CitadelNPCVData
    // Field count: 30
    pub mod CNPC_Boss_Tier2VData {
        pub const m_flPlayerInitialSightRange: usize = 0x10E8; //
        pub const m_strWIPModelName: usize = 0x10F0; //
        pub const m_vecWeakPoints: usize = 0x11D0; //
        pub const m_BeamChargingEffect: usize = 0x11E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BeamPreviewEffect: usize = 0x12C8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BeamActiveEffect: usize = 0x13A8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StompImpactEffect: usize = 0x1488; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StompWarningEffect: usize = 0x1568; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flTossSpeed: usize = 0x1648; //
        pub const m_flStompDamage: usize = 0x164C; //
        pub const m_flStompDamageMaxHealthPercent: usize = 0x1650; //
        pub const m_flStompTossUpMagnitude: usize = 0x1654; //
        pub const m_flStunDuration: usize = 0x1658; //
        pub const m_flStompImpactRadius: usize = 0x165C; //
        pub const m_flStompImpactHeight: usize = 0x1660; //
        pub const m_flSweepRadius: usize = 0x1664; //
        pub const m_flSweepSpeed: usize = 0x1668; //
        pub const m_flSweepZScale: usize = 0x166C; //
        pub const m_flSweepMaxAngle: usize = 0x1670; //
        pub const m_flSweepMaxRange: usize = 0x1674; //
        pub const m_flSweepAdjustSpeed: usize = 0x1678; //
        pub const m_flBurstDuration: usize = 0x167C; //
        pub const m_flBurstCooldown: usize = 0x1680; //
        pub const m_BackdoorProtectionModifier: usize = 0x1688; //
        pub const m_flBackDoorProtectionRange: usize = 0x1698; //
        pub const m_InvulModifier: usize = 0x16A0; //
        pub const m_flInvulModifierRange: usize = 0x16B0; //
        pub const m_RangedArmorModifier: usize = 0x16B8; //
        pub const m_FriendlyAuraModifier: usize = 0x16C8; //
        pub const m_NearbyEnemyResist: usize = 0x16D8; //
    }
    // Parent: C_SoundAreaEntityBase
    // Field count: 2
    pub mod C_SoundAreaEntityOrientedBox {
        pub const m_vMin: usize = 0x588; //
        pub const m_vMax: usize = 0x594; //
    }
    // Parent: CCitadel_Ability_PrimaryWeapon_BeamWeapon
    // Field count: 3
    pub mod CCitadel_Ability_PrimaryWeapon_Bebop {
        pub const m_flStartWindUpTime: usize = 0x1030; //
        pub const m_flStartFiringTime: usize = 0x1034; //
        pub const m_bFiring: usize = 0x1038; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_SiphonBullets_HealthLoss {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_Upgrade_ArcaneSurge_VData {
        pub const m_WeaponBuffModifier: usize = 0x638; //
        pub const m_AbilityWatcherModifier: usize = 0x648; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_LongRangeSlowingTech_ProcVData {
        pub const m_DebuffModifier: usize = 0x668; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierDelayedStunVData {
        pub const m_HitParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 0
    pub mod C_Citadel_CatAnimating {
    }
    // Parent: C_BaseModelEntity
    // Field count: 8
    pub mod C_TextureBasedAnimatable {
        pub const m_bLoop: usize = 0x858; //
        pub const m_flFPS: usize = 0x85C; //
        pub const m_hPositionKeys: usize = 0x860; //
        pub const m_hRotationKeys: usize = 0x868; //
        pub const m_vAnimationBoundsMin: usize = 0x870; //
        pub const m_vAnimationBoundsMax: usize = 0x87C; //
        pub const m_flStartTime: usize = 0x888; //
        pub const m_flStartFrame: usize = 0x88C; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_IdolReturnTimer {
    }
    // Parent: CCitadel_Modifier_ScalingPowerUp
    // Field count: 0
    pub mod CCitadel_Modifier_PowerUp_Gun {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ShadowClone {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Targetdummy_2 {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CModifierRiotProtocolBuffVData {
        pub const m_LaserParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PulseHitEnemyParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyDebuffModifier: usize = 0x7F8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierCrowdControlDebuffVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 4
    pub mod CCitadel_Modifier_GarbageAuraTarget {
        pub const m_flMaxDist: usize = 0x138; //
        pub const m_vecOffsetDir: usize = 0x13C; //
        pub const m_vecStartPosition: usize = 0x148; //
        pub const m_flAOERadius: usize = 0x154; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_AirRaid {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadelModifierIdolReturnTimerVData {
        pub const m_ChannelParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_IcePath_TechPowerLinger {
        pub const m_nBonusSpirit: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_VeilWalkerWatcherVData {
        pub const m_InvisModifier: usize = 0x638; //
        pub const m_VeilWalkerTriggeredModifier: usize = 0x648; //
        pub const m_VeilWalkerMovespeed: usize = 0x658; //
        pub const m_flTraceLengthMin: usize = 0x668; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_MeleeTarget {
    }
    // Parent: CNPC_TrooperNeutralVData
    // Field count: 4
    pub mod CNPC_TrooperNeutralNodeMoverVData {
        pub const m_bEnableMovementToNodes: usize = 0x1450; //
        pub const m_flExposedDuration: usize = 0x1454; //
        pub const m_flHideDuration: usize = 0x145C; //
        pub const m_HidingModifier: usize = 0x1468; //
    }
    // Parent: C_SoundOpvarSetPointBase
    // Field count: 0
    pub mod C_SoundOpvarSetPointEntity {
    }
    // Parent: C_NPC_TrooperNeutral
    // Field count: 0
    pub mod C_NPC_MidBossHeroTest {
    }
    // Parent: C_LightEntity
    // Field count: 0
    pub mod C_LightOrthoEntity {
    }
    // Parent: CCitadel_Modifier_PowerUp
    // Field count: 0
    pub mod CCitadel_Modifier_PermanentPickup {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CModifier_Mirage_Tornado_Aura_Apply_VData {
        pub const m_TossModifier: usize = 0x638; //
        pub const m_LiftModifier: usize = 0x648; //
        pub const m_strHitConfirmSound: usize = 0x658; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityThumper3VData {
        pub const m_DroneModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_ShakedownPulseVData {
        pub const m_strFireSound: usize = 0x638; //
        pub const m_ShakeParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChainParticle: usize = 0x728; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_WreckerScrapBlastDebuff {
        pub const m_flEnemyMoveSlow: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_HealingPulse_Tracker {
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_Item_TechCleaveVData {
        pub const m_TechCleaveModifier: usize = 0x15D8; //
        pub const m_sCleaveProcSound: usize = 0x15E8; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_RescueBeam {
        pub const m_flHealthPerTick: usize = 0x1A0; //
        pub const m_nBeamIndex: usize = 0x1A4; //
    }
    // Parent: CCitadelItemPickupRejuvVData
    // Field count: 0
    pub mod CCitadelItemPickupRejuvHeroTestVData {
    }
    // Parent: C_BaseEntity
    // Field count: 2
    pub mod CSkyboxReference {
        pub const m_worldGroupId: usize = 0x560; //
        pub const m_hSkyCamera: usize = 0x564; //
    }
    // Parent: C_TonemapController2
    // Field count: 0
    pub mod C_TonemapController2Alias_env_tonemap_controller2 {
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 2
    pub mod CCitadelModifierAura_ConeVData {
        pub const m_flAuraTargetingConeHalfWidth: usize = 0x680; //
        pub const m_flAuraTargetingConeAngle: usize = 0x684; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ControlPointCapturerAuraTarget {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Dust_Storm_Thrown {
    }
    // Parent: CitadelItemVData
    // Field count: 4
    pub mod CCitadel_ArmorUpgrade_PersonalRejuvenatorVData {
        pub const m_DeployParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RespawnParticle: usize = 0x16B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sDeploySound: usize = 0x1798; //
        pub const m_sRespawnSound: usize = 0x17A8; //
    }
    // Parent: CCitadelModifier
    // Field count: 18
    pub mod CCitadel_Modifier_Tier2Boss_LaserBeam {
        pub const m_bPreview: usize = 0x168; //
        pub const m_flSoundStartTime: usize = 0x174; //
        pub const m_vStart: usize = 0x17C; //
        pub const m_vEnd: usize = 0x188; //
        pub const m_vPrevEnd: usize = 0x194; //
        pub const m_flAngleBetweenTrace: usize = 0x1A0; //
        pub const m_flDamagePerTick: usize = 0x1A4; //
        pub const m_flCreepDamagePerTick: usize = 0x1A8; //
        pub const m_flNextDamageTick: usize = 0x1AC; //
        pub const m_vecEntitiesHit: usize = 0x1B0; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_flDamageTickRate: usize = 0x1C8; //
        pub const m_flLastShakeTime: usize = 0x1CC; //
        pub const m_bSweepRightFirst: usize = 0x1D0; //
        pub const m_angBeamAim: usize = 0x1D4; //
        pub const m_vecBeamTarget: usize = 0x1E0; //
        pub const m_flLastBeamUpdateTime: usize = 0x1EC; //
        pub const m_flTargetingTaskStartTime: usize = 0x208; //
        pub const m_flTrackVel: usize = 0x20C; //
    }
    // Parent: None
    // Field count: 30
    pub mod CProjectedTextureBase {
        pub const m_hTargetEntity: usize = 0xC; //
        pub const m_bState: usize = 0x10; //
        pub const m_bAlwaysUpdate: usize = 0x11; //
        pub const m_flLightFOV: usize = 0x14; //
        pub const m_bEnableShadows: usize = 0x18; //
        pub const m_bSimpleProjection: usize = 0x19; //
        pub const m_bLightOnlyTarget: usize = 0x1A; //
        pub const m_bLightWorld: usize = 0x1B; //
        pub const m_bCameraSpace: usize = 0x1C; //
        pub const m_flBrightnessScale: usize = 0x20; //
        pub const m_LightColor: usize = 0x24; //
        pub const m_flIntensity: usize = 0x28; //
        pub const m_flLinearAttenuation: usize = 0x2C; //
        pub const m_flQuadraticAttenuation: usize = 0x30; //
        pub const m_bVolumetric: usize = 0x34; //
        pub const m_flVolumetricIntensity: usize = 0x38; //
        pub const m_flNoiseStrength: usize = 0x3C; //
        pub const m_flFlashlightTime: usize = 0x40; //
        pub const m_nNumPlanes: usize = 0x44; //
        pub const m_flPlaneOffset: usize = 0x48; //
        pub const m_flColorTransitionTime: usize = 0x4C; //
        pub const m_flAmbient: usize = 0x50; //
        pub const m_SpotlightTextureName: usize = 0x54; //
        pub const m_nSpotlightTextureFrame: usize = 0x254; //
        pub const m_nShadowQuality: usize = 0x258; //
        pub const m_flNearZ: usize = 0x25C; //
        pub const m_flFarZ: usize = 0x260; //
        pub const m_flProjectionSize: usize = 0x264; //
        pub const m_flRotation: usize = 0x268; //
        pub const m_bFlipHorizontal: usize = 0x26C; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_SleepAOE {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_WeaponUpgrade_FireRateAura {
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProcVData
    // Field count: 6
    pub mod CCitadel_Modifier_EmpowerBulletVData {
        pub const m_DebuffModifier: usize = 0x768; //
        pub const m_ExplosionParticle: usize = 0x778; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionVictimParticle: usize = 0x858; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EmpowerWeaponParticle: usize = 0x938; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShotVictimSound: usize = 0xA18; //
        pub const m_ShotConfirmationSound: usize = 0xA28; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_AirLiftExplodingAllyVData {
        pub const m_strExplodeEffect: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AOEModifier: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Bebop_LaserBeamVData {
        pub const m_LaserModifier: usize = 0x1590; //
        pub const m_ChargeParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flCancelCooldown: usize = 0x1680; //
    }
    // Parent: CCitadel_Modifier_Invis
    // Field count: 1
    pub mod CCitadel_Modifier_Camouflage_Invis {
        pub const m_vCastPosition: usize = 0x268; //
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 0
    pub mod CCitadel_Ability_TrooperGrenade {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_BaseShield {
    }
    // Parent: C_NPC_TrooperNeutral
    // Field count: 0
    pub mod C_NPC_TrooperNeutralNodeMover {
    }
    // Parent: CAttributeManager
    // Field count: 1
    pub mod CAttributeContainer {
        pub const m_Item: usize = 0x68; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_AnimalCurseVData {
        pub const m_CursedModel: usize = 0x638; //
        pub const m_TargetParticle: usize = 0x720; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flModelScale: usize = 0x800; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Lash_Flog_Debuff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_FlameDashVData {
        pub const m_GroundAuraModifier: usize = 0x638; //
        pub const m_ProgressModifier: usize = 0x648; //
        pub const m_FlameDashParticle: usize = 0x658; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FlameAuraParticle: usize = 0x738; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Stabilizing_Tripod {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_VexBarrierVData {
        pub const m_ShieldModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_HollowPoint_Stack {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 4
    pub mod CCitadel_Modifier_SpiritSnatch_VData {
        pub const m_BuffModifier: usize = 0x668; //
        pub const m_DebuffModifier: usize = 0x678; //
        pub const m_SwingParticle: usize = 0x688; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x768; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_SlowImmunity {
    }
    // Parent: C_PathParticleRope
    // Field count: 0
    pub mod C_PathParticleRopeAlias_path_particle_rope_clientside {
    }
    // Parent: CBaseFilter
    // Field count: 4
    pub mod CFilterMultiple {
        pub const m_nFilterType: usize = 0x5B8; //
        pub const m_iFilterName: usize = 0x5C0; //
        pub const m_hFilter: usize = 0x610; //
        pub const m_nFilterCount: usize = 0x638; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Fear_VData {
        pub const m_ExplodeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_Magician_BigBolt {
        pub const m_flNextShootTime: usize = 0xEA0; //
        pub const m_iBoltsFired: usize = 0xEA4; //
        pub const m_iRemainingBolts: usize = 0xEA8; //
        pub const m_bPreppingShoot: usize = 0xEAC; //
    }
    // Parent: CCitadel_Modifier_Intrinsic_Base
    // Field count: 0
    pub mod CCitadel_Modifier_MagicStormWatcher {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_DiscordVData {
        pub const m_ImpactParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TechRangeClamp {
    }
    // Parent: None
    // Field count: 2
    pub mod CitadelStolenAbilitySlot_t {
        pub const m_eStolenSlot: usize = 0x8; //
        pub const m_bIsActivelyStolen: usize = 0xA; //
    }
    // Parent: CPlayerPawnComponent
    // Field count: 0
    pub mod CPlayer_UseServices {
    }
    // Parent: C_Citadel_BreakblePropModifierPickup
    // Field count: 0
    pub mod C_Citadel_PunchablePowerup {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Synth_Grasp_Caster_VData {
        pub const m_CastParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 1
    pub mod CCitadel_Modifier_Petrify {
        pub const flAccumulatedDamage: usize = 0xC8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityRestorativeGooVData {
        pub const m_RestorativeGooParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RestorativeGooModifier: usize = 0x1670; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_Teleport {
        pub const m_bTeleportingToTarget: usize = 0xCA0; //
        pub const m_vTargetPosition: usize = 0xCA4; //
        pub const m_vTargetAngles: usize = 0xCB0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_BulletArmorReductionVData {
    }
    // Parent: CEntityComponent
    // Field count: 1
    pub mod CScriptComponent {
        pub const m_scriptClassName: usize = 0x30; //
    }
    // Parent: C_BaseEntity
    // Field count: 17
    pub mod C_EnvLightProbeVolume {
        pub const m_Entity_hLightProbeTexture: usize = 0x1540; //
        pub const m_Entity_hLightProbeDirectLightIndicesTexture: usize = 0x1548; //
        pub const m_Entity_hLightProbeDirectLightScalarsTexture: usize = 0x1550; //
        pub const m_Entity_hLightProbeDirectLightShadowsTexture: usize = 0x1558; //
        pub const m_Entity_vBoxMins: usize = 0x1560; //
        pub const m_Entity_vBoxMaxs: usize = 0x156C; //
        pub const m_Entity_bMoveable: usize = 0x1578; //
        pub const m_Entity_nHandshake: usize = 0x157C; //
        pub const m_Entity_nPriority: usize = 0x1580; //
        pub const m_Entity_bStartDisabled: usize = 0x1584; //
        pub const m_Entity_nLightProbeSizeX: usize = 0x1588; //
        pub const m_Entity_nLightProbeSizeY: usize = 0x158C; //
        pub const m_Entity_nLightProbeSizeZ: usize = 0x1590; //
        pub const m_Entity_nLightProbeAtlasX: usize = 0x1594; //
        pub const m_Entity_nLightProbeAtlasY: usize = 0x1598; //
        pub const m_Entity_nLightProbeAtlasZ: usize = 0x159C; //
        pub const m_Entity_bEnabled: usize = 0x15A9; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityGenericPerson4VData {
    }
    // Parent: CCitadel_Modifier_Sleep
    // Field count: 0
    pub mod CCitadel_Modifier_SleepDagger_Asleep {
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_Chrono_KineticCarbine {
        pub const m_nBulletCount: usize = 0xC0; //
        pub const m_flElapsedPct: usize = 0xC4; //
        pub const m_hTimeWarp: usize = 0xC8; // CHandle<CCitadelBulletTimeWarp>
        pub const m_nFullyChargedParticle: usize = 0xCC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_HealthSwapPrecastVData {
        pub const m_strTargetParticleEffect: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strTargetEnemyParticleEffect: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strTargetScreenParticleEffect: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_LifeDrainVData {
        pub const m_SilenceModifier: usize = 0x638; //
        pub const m_DrainParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_BeltFed_MagazineVData {
        pub const m_SpinUpSound: usize = 0x638; //
        pub const m_SpinDownSound: usize = 0x648; //
        pub const m_SpinLoopSound: usize = 0x658; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CItemStimPakVData {
        pub const m_StimPakModifier: usize = 0x15D8; //
        pub const m_CastParticle: usize = 0x15E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_TriggerPush {
        pub const m_vPush: usize = 0xC0; //
    }
    // Parent: None
    // Field count: 12
    pub mod STeamFOWEntity {
        pub const m_nEntIndex: usize = 0x30; //
        pub const m_nTeam: usize = 0x34; //
        pub const m_eClass: usize = 0x38; //
        pub const m_iLane: usize = 0x3C; //
        pub const m_eHeight: usize = 0x40; //
        pub const m_bVisibleOnMap: usize = 0x41; //
        pub const m_bBackdoorProtectionActive: usize = 0x42; //
        pub const m_nTickHidden: usize = 0x44; //
        pub const m_strEntityName: usize = 0x48; //
        pub const m_nHealthPercent: usize = 0x50; //
        pub const m_nPositionX: usize = 0x51; //
        pub const m_nPositionY: usize = 0x52; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 2
    pub mod CCitadel_Modifier_ThermalDetonator_ThinkerVData {
        pub const m_GroundParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_GroundParticleFriendly: usize = 0x760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CCitadel_Ability_ViperHookBladeVData {
        pub const m_SlowDebuffModifier: usize = 0x1590; //
        pub const m_DaggerStuckParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerImpactParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerExplodeParticle: usize = 0x1760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDaggerHitSound: usize = 0x1840; //
        pub const m_strDaggerExplodeSound: usize = 0x1850; //
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_FireBomb {
        pub const m_flSideMoveSpeed: usize = 0x130; //
        pub const m_vReturnPosition: usize = 0x134; //
        pub const m_vReturnAngles: usize = 0x140; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Item_AOESilence_Target {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_TechBurst_ProcVData {
        pub const m_ProcParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_RespawnCredit {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_HealthRegenAura {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_CitadelTrackedProjectile {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_FearWatcherVData {
        pub const m_BuildupProcModifier: usize = 0x668; //
        pub const m_BuildUpModifier: usize = 0x678; //
        pub const m_ExplodeSound: usize = 0x688; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Cadence_Crescendo_InAOE_VData {
        pub const m_PostAOEModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_VeilWalkerMovespeed {
    }
    // Parent: CCitadel_Modifier_Intrinsic_Base
    // Field count: 1
    pub mod CCitadel_Modifier_ReinforcingCasings {
        pub const m_LastHitShotID: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Upgrade_OverdriveClip_VData {
        pub const m_BuffEffect: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TracerParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 4
    pub mod CCitadel_Modifier_DisarmProcWatcherVData {
        pub const m_BuildUpModifier: usize = 0x668; //
        pub const m_DisarmProcModifier: usize = 0x678; //
        pub const m_ImmunityModifier: usize = 0x688; //
        pub const m_TracerParticle: usize = 0x698; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_DiminishingSlow {
        pub const m_flSlowPercent: usize = 0xC0; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_UtilityUpgrade_DebuffImmunity {
    }
    // Parent: C_BaseEntity
    // Field count: 1
    pub mod CRagdollManager {
        pub const m_iCurrentMaxRagdollCount: usize = 0x560; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CCitadel_Ability_Magician_EscapeVData {
        pub const m_EscapedModifier: usize = 0x1590; //
        pub const m_PoofParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TetherParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strEscaped: usize = 0x1760; //
        pub const m_cameraSequenceTeleport: usize = 0x1770; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbility_Rutger_CheatDeath_VData {
        pub const m_ModifierCheatDeathActivated: usize = 0x1590; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierFealtyTargetVData {
        pub const m_CastParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_Arcane_Eater_Proc {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CItemMetalSkinVData {
        pub const m_MetalSkinModifier: usize = 0x15D8; //
    }
    // Parent: CitadelItemVData
    // Field count: 3
    pub mod CCitadel_Upgrade_AmmoScavenger_VData {
        pub const m_BuffModifier: usize = 0x15D8; //
        pub const m_StackSound: usize = 0x15E8; //
        pub const m_AmmoSound: usize = 0x15F8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 7
    pub mod CCitadel_Modifier_ShieldTracker_BaseVData {
        pub const m_flShieldImpactEffectDuration: usize = 0x638; //
        pub const m_ShieldImpactParticle: usize = 0x640; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShieldImpactModifier: usize = 0x720; //
        pub const eShieldType: usize = 0x730; //
        pub const flCooldownOnDamage: usize = 0x734; //
        pub const flCooldownOnBreak: usize = 0x738; //
        pub const flRegenDuration: usize = 0x73C; //
    }
    // Parent: C_BaseEntity
    // Field count: 30
    pub mod C_TeamRoundTimer {
        pub const m_bTimerPaused: usize = 0x560; //
        pub const m_flTimeRemaining: usize = 0x564; //
        pub const m_flTimerEndTime: usize = 0x568; //
        pub const m_bIsDisabled: usize = 0x56C; //
        pub const m_bShowInHUD: usize = 0x56D; //
        pub const m_nTimerLength: usize = 0x570; //
        pub const m_nTimerInitialLength: usize = 0x574; //
        pub const m_nTimerMaxLength: usize = 0x578; //
        pub const m_bAutoCountdown: usize = 0x57C; //
        pub const m_nSetupTimeLength: usize = 0x580; //
        pub const m_nState: usize = 0x584; //
        pub const m_bStartPaused: usize = 0x588; //
        pub const m_bInCaptureWatchState: usize = 0x589; //
        pub const m_flTotalTime: usize = 0x58C; //
        pub const m_bStopWatchTimer: usize = 0x590; //
        pub const m_bFireFinished: usize = 0x591; //
        pub const m_bFire5MinRemain: usize = 0x592; //
        pub const m_bFire4MinRemain: usize = 0x593; //
        pub const m_bFire3MinRemain: usize = 0x594; //
        pub const m_bFire2MinRemain: usize = 0x595; //
        pub const m_bFire1MinRemain: usize = 0x596; //
        pub const m_bFire30SecRemain: usize = 0x597; //
        pub const m_bFire10SecRemain: usize = 0x598; //
        pub const m_bFire5SecRemain: usize = 0x599; //
        pub const m_bFire4SecRemain: usize = 0x59A; //
        pub const m_bFire3SecRemain: usize = 0x59B; //
        pub const m_bFire2SecRemain: usize = 0x59C; //
        pub const m_bFire1SecRemain: usize = 0x59D; //
        pub const m_nOldTimerLength: usize = 0x5A0; //
        pub const m_nOldTimerState: usize = 0x5A4; //
    }
    // Parent: CCitadelModelEntity
    // Field count: 1
    pub mod C_LaneNode {
        pub const m_nPlayerTeamEventIndex: usize = 0x890; //
    }
    // Parent: CCitadelModelEntity
    // Field count: 1
    pub mod C_CitadelViscousBall {
        pub const m_hAbility: usize = 0x860; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CModifier_Operative_Revelation_Target {
        pub const m_flTotalTimeLookedAtCaster: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 6
    pub mod CModifier_Operative_UmbrellaManeuver_AirHang_VData {
        pub const m_ExplodeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TracerParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x7F8; //
        pub const m_flAirDrag: usize = 0x808; //
        pub const m_flAirSpeed: usize = 0x80C; //
        pub const m_flFallSpeed: usize = 0x810; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbilityThumper2VData {
        pub const m_StompParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strStompExplosionSound: usize = 0x1670; //
        pub const m_BuffModifier: usize = 0x1680; //
        pub const m_BarbedWireAuraModifier: usize = 0x1690; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_HauntWatcherVData {
        pub const m_HauntDamageModifier: usize = 0x668; //
        pub const m_BuildUpModifier: usize = 0x678; //
        pub const m_ExplodeSound: usize = 0x688; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ExplosiveBarrel {
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CAbilityVacuumVData {
        pub const m_VacuumAuraModifier: usize = 0x1590; //
        pub const m_flAirSpeedMax: usize = 0x15A0; //
        pub const m_flFallSpeedMax: usize = 0x15A4; //
        pub const m_flAirDrag: usize = 0x15A8; //
        pub const m_flMaxMovespeed: usize = 0x15AC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierBullChargingVData {
        pub const m_ChargeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbilitySprintVData {
        pub const m_SprintParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strSprintSound: usize = 0x1670; //
        pub const m_flInCombatDuration: usize = 0x1680; //
        pub const m_flSprintAccMS: usize = 0x1684; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_ZiplineSpeed {
        pub const m_iLane: usize = 0xC0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CAbility_Synth_Affliction {
        pub const m_nBonusTechPower: usize = 0xCA0; //
        pub const m_hAOEParticle: usize = 0xD50; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Tenacity {
    }
    // Parent: CCitadelYamatoBaseVData
    // Field count: 12
    pub mod CCitadel_Ability_InfinitySlashVData {
        pub const m_flRiseSpeed: usize = 0x1598; //
        pub const m_flRiseDuration: usize = 0x159C; //
        pub const m_flSpeedDecayScale: usize = 0x15A0; //
        pub const m_flExplodeHoldTime: usize = 0x15A4; //
        pub const m_flExplosionShakeAmplitude: usize = 0x15A8; //
        pub const m_flExplosionShakeFrequency: usize = 0x15AC; //
        pub const m_flExplosionShakeDuration: usize = 0x15B0; //
        pub const m_AOERangeEffect: usize = 0x15B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AnimCastEffect: usize = 0x1698; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_cameraSequenceExplosion: usize = 0x1778; //
        pub const m_BuffModifier: usize = 0x1800; //
        pub const m_BuffTimerModifier: usize = 0x1810; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierRiotCastDelayVData {
        pub const m_UnstoppableModifier: usize = 0x638; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 16
    pub mod CCitadel_Ability_TangoTether {
        pub const m_desatVolIdx: usize = 0xCA0; //
        pub const m_vecCastStartPos: usize = 0xCA4; //
        pub const m_vecDashStartPos: usize = 0xCB0; //
        pub const m_vecDashEndPos: usize = 0xCBC; //
        pub const m_angDashStartAng: usize = 0xCC8; //
        pub const m_flDashStartTime: usize = 0xCD4; //
        pub const m_flGrappleStartTime: usize = 0xCD8; //
        pub const m_flGrappleArriveTime: usize = 0xCDC; //
        pub const m_hTarget: usize = 0xCE0; //
        pub const m_flVelSpring: usize = 0xCE4; //
        pub const m_flGrappleShotAttackTime: usize = 0xCE8; //
        pub const m_nTicksNotMoving: usize = 0xCEC; //
        pub const m_vecPrevPos: usize = 0xCF0; //
        pub const m_rgTargetPos: usize = 0xCFC; //
        pub const m_rgTargetPosTime: usize = 0xDEC; //
        pub const m_nGrappleTravelEffect: usize = 0xE3C; //
    }
    // Parent: CCitadelBaseShivAbility
    // Field count: 1
    pub mod CCitadel_Ability_ShivDagger {
        pub const m_bIsInRicochet: usize = 0xCA0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Puddle {
    }
    // Parent: CitadelAbilityVData
    // Field count: 24
    pub mod CCitadel_Ability_Bull_LeapVData {
        pub const m_CrashSpeedScaleCurve: usize = 0x1590; //
        pub const m_BoostModifier: usize = 0x15D0; //
        pub const m_CrashModifier: usize = 0x15E0; //
        pub const m_ImmunityModifier: usize = 0x15F0; //
        pub const m_LandingBonusesModifier: usize = 0x1600; //
        pub const m_TakeOffParticle: usize = 0x1610; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x16F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AoEPreviewParticle: usize = 0x17D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_nHoverParticle: usize = 0x18B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strCrashingSound: usize = 0x1990; //
        pub const m_strImpactSound: usize = 0x19A0; //
        pub const m_flStartupTime: usize = 0x19B0; //
        pub const m_flForwardBoostSpeed: usize = 0x19B4; //
        pub const m_flUpBoostSpeed: usize = 0x19B8; //
        pub const m_flBoostTurnRate: usize = 0x19BC; //
        pub const m_flHoverTime: usize = 0x19C0; //
        pub const m_flMinAimAngle: usize = 0x19C4; //
        pub const m_flBoostGain: usize = 0x19C8; //
        pub const m_flBoostTime: usize = 0x19CC; //
        pub const m_flLandingTime: usize = 0x19D0; //
        pub const m_flCrashSpeed: usize = 0x19D4; //
        pub const m_flHoverInputSpeedMax: usize = 0x19D8; //
        pub const m_flHoverInputAcceleration: usize = 0x19DC; //
        pub const m_flHoverSpeedDecay: usize = 0x19E0; //
    }
    // Parent: C_BaseEntity
    // Field count: 32
    pub mod C_CitadelBaseAbility {
        pub const m_vecIntrinsicModifiers: usize = 0x630; //
        pub const m_pCastDelayAutoModifier: usize = 0x648; //
        pub const m_pChannelAutoModifier: usize = 0x660; //
        pub const m_strUsedCastGraphParam: usize = 0x678; //
        pub const m_nCastParamNeedsResetTick: usize = 0x680; //
        pub const m_bIsCoolingDownInternal: usize = 0x684; //
        pub const m_flCancelLockoutTime: usize = 0x688; //
        pub const m_bChanneling: usize = 0x6A8; //
        pub const m_bInCastDelay: usize = 0x6A9; //
        pub const m_vecImbuedByAbilitiyIDs: usize = 0x6B0; // C_NetworkUtlVectorBase<CUtlStringToken>
        pub const m_bCanBeUpgraded: usize = 0x6C8; //
        pub const m_eStolenInSlot: usize = 0x6D0; //
        pub const m_nUpgradeBits: usize = 0x6E0; //
        pub const m_iBucketID: usize = 0x6E4; //
        pub const m_bToggleState: usize = 0x6E8; //
        pub const m_flCooldownStart: usize = 0x6EC; //
        pub const m_flCooldownEnd: usize = 0x6F0; //
        pub const m_flCastCompletedTime: usize = 0x6F4; //
        pub const m_flChannelStartTime: usize = 0x6F8; //
        pub const m_flCastDelayStartTime: usize = 0x6FC; //
        pub const m_eAbilitySlot: usize = 0x700; //
        pub const m_flPostCastDelayEndTime: usize = 0x704; //
        pub const m_iRemainingCharges: usize = 0x708; //
        pub const m_flChargeRechargeStart: usize = 0x70C; //
        pub const m_flChargeRechargeEnd: usize = 0x710; //
        pub const m_flMovementControlActiveTime: usize = 0x714; //
        pub const m_flSelectedChangedTime: usize = 0x718; //
        pub const m_flAltCastHoldStartTime: usize = 0x71C; //
        pub const m_flAltCastDoubleTapStartTime: usize = 0x720; //
        pub const m_bCanBeImbued: usize = 0x724; //
        pub const m_nImbuedAbilityID: usize = 0x728; //
        pub const m_bSelectionModeIsAltMode: usize = 0x72C; //
    }
    // Parent: C_NPC_SimpleAnimatingAI
    // Field count: 1
    pub mod C_NPC_TeslaCoil {
        pub const m_CCitadelAbilityComponent: usize = 0xB00; //
    }
    // Parent: C_ModelPointEntity
    // Field count: 0
    pub mod C_EnvProjectedTexture {
    }
    // Parent: C_BaseEntity
    // Field count: 2
    pub mod CPathSimple {
        pub const m_CPathQueryComponent: usize = 0x568; //
        pub const m_pathString: usize = 0x618; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_MageWalkVData {
        pub const m_BubbleModifier: usize = 0x1590; //
        pub const m_TurretModifier: usize = 0x15A0; //
        pub const m_strCastEffect: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 8
    pub mod CAbility_Fathom_LurkersAmbush {
        pub const m_ChargeUpParticle: usize = 0xCA0; //
        pub const m_hRegenModifier: usize = 0xE68; //
        pub const m_hInvisModifier: usize = 0xE80; //
        pub const m_bIsVisibleOnMinimap: usize = 0xE98; //
        pub const m_flStoppedMovingStartTime: usize = 0xE9C; //
        pub const m_vLastPos: usize = 0xEA0; //
        pub const m_flImmobilizeDuration: usize = 0xEAC; //
        pub const m_flChannelTimeStarted: usize = 0xEB0; //
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 1
    pub mod CCitadel_Modifier_UltCombo_Target {
        pub const m_angles: usize = 0xC8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_WreckingBall_AutoThrow {
    }
    // Parent: CCitadelModifier
    // Field count: 7
    pub mod CCitadel_Modifier_Bebop_LaserBeam {
        pub const m_flSoundStartTime: usize = 0x330; //
        pub const m_vStart: usize = 0x338; //
        pub const m_vEnd: usize = 0x344; //
        pub const m_vPrevEnd: usize = 0x350; //
        pub const m_flAngleBetweenTrace: usize = 0x35C; //
        pub const m_flDamagePerTick: usize = 0x360; //
        pub const m_flNextDamageTick: usize = 0x364; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Astro_Rifle_SelfVData {
        pub const m_WeaponFxParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Kelvin_Frozen {
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_Chrono_KineticCarbineVData {
        pub const m_flShotTimeScaleLingerDuration: usize = 0x1590; //
        pub const m_ChargingModifier: usize = 0x1598; //
        pub const m_DebuffModifier: usize = 0x15A8; //
        pub const m_cameraKineticCarbineShotFired: usize = 0x15B8; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_Item_DivinersKevlar_VData {
        pub const m_BuffModifier: usize = 0x15D8; //
        pub const m_PrecastSpiritBuffModifier: usize = 0x15E8; //
    }
    // Parent: CCitadel_Item
    // Field count: 3
    pub mod CCitadel_Upgrade_MagicCarpet {
        pub const m_flFlyingStartTime: usize = 0xCB8; //
        pub const m_bFlying: usize = 0xD68; //
        pub const m_bSummoning: usize = 0xD69; //
    }
    // Parent: C_PointClientUIWorldPanel
    // Field count: 4
    pub mod CPointOffScreenIndicatorUi {
        pub const m_bBeenEnabled: usize = 0xAC0; //
        pub const m_bHide: usize = 0xAC1; //
        pub const m_flSeenTargetTime: usize = 0xAC4; //
        pub const m_pTargetPanel: usize = 0xAC8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Shakedown_Target {
        pub const m_hShadowdownAbility: usize = 0xCA0; //
        pub const m_AimPos: usize = 0xCA4; //
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 8
    pub mod CCitadel_Bounce_PadVData {
        pub const m_sModelName: usize = 0x28; //
        pub const m_IdleParticle: usize = 0x108; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BounceParticle: usize = 0x1E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DestroyParticle: usize = 0x2C8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strCasterBounceSound: usize = 0x3A8; //
        pub const m_strOtherHeroBounceSound: usize = 0x3B8; //
        pub const m_strBarrelBounceSound: usize = 0x3C8; //
        pub const m_strExpiredSound: usize = 0x3D8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_SmokeBombVData {
        pub const m_InvisModifier: usize = 0x1590; //
        pub const m_BuffModifier: usize = 0x15A0; //
        pub const m_PhaseOutModifier: usize = 0x15B0; //
        pub const m_PurgeParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 12
    pub mod CCitadel_Modifier_BurstFire_Actuator {
        pub const m_bLastShotInFlight: usize = 0xC0; //
        pub const m_bBonusTracked: usize = 0xC1; //
        pub const m_nHitCounter: usize = 0xC4; //
        pub const m_nTotalBurstFireShots: usize = 0xC8; //
        pub const m_nInitialzedClipSize: usize = 0xCC; //
        pub const m_nBonusPitch: usize = 0xD0; //
        pub const m_bInitialized: usize = 0xD4; //
        pub const m_nIncreasedBurstShotCount: usize = 0xD8; //
        pub const m_flIntraBurstCycleTime: usize = 0xDC; //
        pub const m_flCycleTimePct: usize = 0xE0; //
        pub const m_flMaxCycleTimeOverride: usize = 0xE4; //
        pub const m_flMaxBurstFireCooldownOverride: usize = 0xE8; //
    }
    // Parent: CCitadel_Modifier_StatStealBaseVData
    // Field count: 1
    pub mod CCitadel_Modifier_Siphon_Bullets_WatcherVData {
        pub const m_HealModifier: usize = 0x658; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CItemSilenceGlyphVData {
        pub const m_DebuffModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Upgrade_Magic_Storm {
    }
    // Parent: CPlayer_MovementServices
    // Field count: 2
    pub mod CCitadelObserver_MovementServices {
        pub const m_flRoamingSpeed: usize = 0x1D8; //
        pub const m_bHasFreeCursor: usize = 0x1DC; //
    }
    // Parent: CBodyComponentSkeletonInstance
    // Field count: 0
    pub mod CBodyComponentBaseModelEntity {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_Trapper_FearVData {
        pub const m_ImpactParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffModifier: usize = 0x1670; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_IceDome_AuraModifierBase {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SilenceProc_Debuff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TechCleave {
    }
    // Parent: None
    // Field count: 76
    pub mod CitadelHeroData_t {
        pub const m_vecAnimGraphDefaultValueOverrides: usize = 0x8; //
        pub const m_HeroID: usize = 0x28; //
        pub const m_hDamageTakenParticle: usize = 0x30; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_hGroundDamageTakenParticle: usize = 0x110; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_hDeathParticle: usize = 0x1F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_hLowHealthParticle: usize = 0x2D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strSelectionImage: usize = 0x3B0; //
        pub const m_strIconImageSmall: usize = 0x3C0; //
        pub const m_strIconHeroCard: usize = 0x3D0; //
        pub const m_strMinimapImage: usize = 0x3E0; //
        pub const m_strTopBarImage: usize = 0x3F0; //
        pub const m_strTopBarVertical: usize = 0x400; //
        pub const m_hRespawnParticle: usize = 0x410; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_colorUI: usize = 0x4F0; //
        pub const m_colorGlowFriendly: usize = 0x4F4; //
        pub const m_colorGlowEnemy: usize = 0x4F8; //
        pub const m_colorGlowTeam1: usize = 0x4FC; //
        pub const m_colorGlowTeam2: usize = 0x500; //
        pub const m_strModelName: usize = 0x508; //
        pub const m_nModelSkin: usize = 0x5E8; //
        pub const m_strWIPModelName: usize = 0x5F0; //
        pub const m_strMainOnlyModelName: usize = 0x6D0; //
        pub const m_iAddedTime: usize = 0x7B0; //
        pub const m_sAG2VariationName: usize = 0x7B8; //
        pub const m_strUIPortraitMap: usize = 0x898; //
        pub const m_strUIShoppingMap: usize = 0x8A0; //
        pub const m_heroStatsUI: usize = 0x8A8; //
        pub const m_heroStatsDisplay: usize = 0x8D8; //
        pub const m_ShopStatDisplay: usize = 0x968; //
        pub const m_strDeathSound: usize = 0xA10; //
        pub const m_strLastHitSound: usize = 0xA20; //
        pub const m_strRosterSelectedSound: usize = 0xA30; //
        pub const m_strRosterRemovedSound: usize = 0xA40; //
        pub const m_strFootstepSoundEventDefault: usize = 0xA50; //
        pub const m_strLowHealthSound: usize = 0xA60; //
        pub const m_strHeroSpecificLowHealthSound: usize = 0xA70; //
        pub const m_strMovementLoop: usize = 0xA80; //
        pub const m_hFootstepSounds: usize = 0xA90; //
        pub const m_hGameSoundEventScript: usize = 0xA98; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCVSoundEventScriptList>>
        pub const m_hGeneratedVOEventScript: usize = 0xB78; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeCVSoundEventScriptList>>
        pub const m_flFootstepSoundTravelDistanceMeters: usize = 0xC58; //
        pub const m_flStealthSpeedMetersPerSecond: usize = 0xC5C; //
        pub const m_flStepSoundTime: usize = 0xC60; //
        pub const m_flStepSoundTimeSprinting: usize = 0xC64; //
        pub const m_flCollisionRadius: usize = 0xC68; //
        pub const m_flCollisionHeight: usize = 0xC6C; //
        pub const m_flStepHeight: usize = 0xC70; //
        pub const m_bInDevelopment: usize = 0xC74; //
        pub const m_bAssignedPlayersOnly: usize = 0xC75; //
        pub const m_bBotSelectable: usize = 0xC76; //
        pub const m_bNewPlayerRecommended: usize = 0xC77; //
        pub const m_bLaneTestingRecommended: usize = 0xC78; //
        pub const m_bNeedsTesting: usize = 0xC79; //
        pub const m_bLimitedTesting: usize = 0xC7A; //
        pub const m_bDisabled: usize = 0xC7B; //
        pub const m_bPlayerSelectable: usize = 0xC7C; //
        pub const m_bAvailableInHeroLabs: usize = 0xC7D; //
        pub const m_nComplexity: usize = 0xC80; //
        pub const m_nReadability: usize = 0xC84; //
        pub const m_flMinLowHealthPercentage: usize = 0xC88; //
        pub const m_flMaxLowHealthPercentage: usize = 0xC8C; //
        pub const m_flMinMidHealthPercentage: usize = 0xC90; //
        pub const m_flMaxMidHealthPercentage: usize = 0xC94; //
        pub const m_flMinHealthForThreshold: usize = 0xC98; //
        pub const m_flMaxHealthForThreshold: usize = 0xC9C; //
        pub const m_mapStartingStats: usize = 0xCA0; // CUtlOrderedMap<EStatsType,float32>
        pub const m_mapScalingStats: usize = 0xCC8; // CUtlOrderedMap<EStatsType,HeroScalingStat_t>
        pub const m_mapBoundAbilities: usize = 0xD08; //
        pub const m_mapWIPAbilities: usize = 0xD30; //
        pub const m_mapItemSlotInfo: usize = 0xD58; //
        pub const m_RecommendedUpgrades: usize = 0xD80; //
        pub const m_RecommendedAbilityOrder: usize = 0xE08; //
        pub const m_eAbilityResourceType: usize = 0xE38; //
        pub const m_mapStandardLevelUpUpgrades: usize = 0xE58; // CUtlOrderedMap<EModifierValue,float32>
        pub const m_mapLevelInfo: usize = 0xE80; // CUtlOrderedMap<int32,HeroLevel_t>
        pub const m_mapPurchaseBonuses: usize = 0xEA8; // CUtlOrderedMap<EItemSlotTypes_t,CUtlVector<HeroPurchaseBonus_t>>
    }
    // Parent: C_BaseEntity
    // Field count: 4
    pub mod C_SkyCamera {
        pub const m_skyboxData: usize = 0x560; //
        pub const m_skyboxSlotToken: usize = 0x5F0; //
        pub const m_bUseAngles: usize = 0x5F4; //
        pub const m_pNext: usize = 0x5F8; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_World {
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 4
    pub mod CModifierVacuumAuraVData {
        pub const m_FinishParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AlliedParticle: usize = 0x760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyParticle: usize = 0x840; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAmbientLoopingLocalPlayerSound: usize = 0x920; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 1
    pub mod C_CitadelProjectile_ImmobilizeTrap {
        pub const m_bShouldDraw: usize = 0x8E0; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 13
    pub mod C_EntityDissolve {
        pub const m_flStartTime: usize = 0x860; //
        pub const m_flFadeInStart: usize = 0x864; //
        pub const m_flFadeInLength: usize = 0x868; //
        pub const m_flFadeOutModelStart: usize = 0x86C; //
        pub const m_flFadeOutModelLength: usize = 0x870; //
        pub const m_flFadeOutStart: usize = 0x874; //
        pub const m_flFadeOutLength: usize = 0x878; //
        pub const m_flNextSparkTime: usize = 0x87C; //
        pub const m_nDissolveType: usize = 0x880; //
        pub const m_vDissolverOrigin: usize = 0x884; //
        pub const m_nMagnitude: usize = 0x890; //
        pub const m_bCoreExplode: usize = 0x894; //
        pub const m_bLinkedToServerEnt: usize = 0x895; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CAbility_Mirage_Tornado {
        pub const m_vLastValidMovementPosition: usize = 0xCA0; //
        pub const m_hActiveProjectile: usize = 0xCAC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierRiotProtocolEnemyDebuffVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityVandalOverflowVData {
        pub const m_LiftModifier: usize = 0x1590; //
        pub const m_TargetParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetCastSound: usize = 0x1680; //
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 2
    pub mod CModifier_Wrecker_UltimateThrowEnemyVData {
        pub const m_EnemyHeroStasisEffect: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyHeroGrabEffect: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_ExplosiveBulletsVData {
        pub const m_ExplodeParticle: usize = 0x768; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x848; //
    }
    // Parent: None
    // Field count: 2
    pub mod StolenAbilityPair_t {
        pub const m_ItemSlotType: usize = 0x30; //
        pub const m_StolenAbilityID: usize = 0x34; //
    }
    // Parent: None
    // Field count: 14
    pub mod C_fogplayerparams_t {
        pub const m_hCtrl: usize = 0x8; //
        pub const m_flTransitionTime: usize = 0xC; //
        pub const m_OldColor: usize = 0x10; //
        pub const m_flOldStart: usize = 0x14; //
        pub const m_flOldEnd: usize = 0x18; //
        pub const m_flOldMaxDensity: usize = 0x1C; //
        pub const m_flOldHDRColorScale: usize = 0x20; //
        pub const m_flOldFarZ: usize = 0x24; //
        pub const m_NewColor: usize = 0x28; //
        pub const m_flNewStart: usize = 0x2C; //
        pub const m_flNewEnd: usize = 0x30; //
        pub const m_flNewMaxDensity: usize = 0x34; //
        pub const m_flNewHDRColorScale: usize = 0x38; //
        pub const m_flNewFarZ: usize = 0x3C; //
    }
    // Parent: C_BaseTrigger
    // Field count: 0
    pub mod C_CitadelIdolReturnTrigger {
    }
    // Parent: C_BaseTrigger
    // Field count: 0
    pub mod C_CitadelClimbRopeTrigger {
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod CFilterTeam {
        pub const m_iFilterTeam: usize = 0x5B8; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_Refresher {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_KnockbackAura {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 4
    pub mod CCitadelItemPickupRejuvVData {
        pub const m_AbilityProjectile: usize = 0x28; //
        pub const m_RejuvModifier: usize = 0x38; //
        pub const m_PunchPickupModifier: usize = 0x48; //
        pub const m_IsFrozenParticle: usize = 0x58; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: None
    // Field count: 29
    pub mod C_EnvWindShared {
        pub const m_flStartTime: usize = 0x8; //
        pub const m_iWindSeed: usize = 0xC; //
        pub const m_iMinWind: usize = 0x10; //
        pub const m_iMaxWind: usize = 0x12; //
        pub const m_windRadius: usize = 0x14; //
        pub const m_iMinGust: usize = 0x18; //
        pub const m_iMaxGust: usize = 0x1A; //
        pub const m_flMinGustDelay: usize = 0x1C; //
        pub const m_flMaxGustDelay: usize = 0x20; //
        pub const m_flGustDuration: usize = 0x24; //
        pub const m_iGustDirChange: usize = 0x28; //
        pub const m_location: usize = 0x2C; //
        pub const m_iszGustSound: usize = 0x38; //
        pub const m_iWindDir: usize = 0x3C; //
        pub const m_flWindSpeed: usize = 0x40; //
        pub const m_currentWindVector: usize = 0x44; //
        pub const m_CurrentSwayVector: usize = 0x50; //
        pub const m_PrevSwayVector: usize = 0x5C; //
        pub const m_iInitialWindDir: usize = 0x68; //
        pub const m_flInitialWindSpeed: usize = 0x6C; //
        pub const m_flVariationTime: usize = 0x70; //
        pub const m_flSwayTime: usize = 0x74; //
        pub const m_flSimTime: usize = 0x78; //
        pub const m_flSwitchTime: usize = 0x7C; //
        pub const m_flAveWindSpeed: usize = 0x80; //
        pub const m_bGusting: usize = 0x84; //
        pub const m_flWindAngleVariation: usize = 0x88; //
        pub const m_flWindSpeedVariation: usize = 0x8C; //
        pub const m_hEntOwner: usize = 0x90; //
    }
    // Parent: C_NPC_HeroCloneTrooper
    // Field count: 0
    pub mod C_NPC_HeroDecoy {
    }
    // Parent: C_BaseTrigger
    // Field count: 12
    pub mod C_PostProcessingVolume {
        pub const m_hPostSettings: usize = 0x9A8; //
        pub const m_flFadeDuration: usize = 0x9B0; //
        pub const m_flMinLogExposure: usize = 0x9B4; //
        pub const m_flMaxLogExposure: usize = 0x9B8; //
        pub const m_flMinExposure: usize = 0x9BC; //
        pub const m_flMaxExposure: usize = 0x9C0; //
        pub const m_flExposureCompensation: usize = 0x9C4; //
        pub const m_flExposureFadeSpeedUp: usize = 0x9C8; //
        pub const m_flExposureFadeSpeedDown: usize = 0x9CC; //
        pub const m_flTonemapEVSmoothingRange: usize = 0x9D0; //
        pub const m_bMaster: usize = 0x9D4; //
        pub const m_bExposureControl: usize = 0x9D5; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_BullCharging {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_LightningBall {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Passive_CloakVData {
        pub const m_InvisModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_UIHudMessage {
        pub const m_strHudMessage: usize = 0xC0; //
        pub const m_bIncludeDecimal: usize = 0xC8; //
        pub const m_eModifierValue: usize = 0xCC; //
        pub const m_flValue: usize = 0xD0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_InShopTunnel {
    }
    // Parent: CPlayerPawnComponent
    // Field count: 0
    pub mod CPlayer_FlashlightServices {
    }
    // Parent: CCitadelItemPickupRejuv
    // Field count: 0
    pub mod CCitadelItemPickupRejuvHeroTest {
    }
    // Parent: CUnitStatusOverlay
    // Field count: 1
    pub mod CUnitStatusOverlayNew {
        pub const m_flUIScale: usize = 0xB28; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod CServerOnlyModelEntity {
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_IcePath {
        pub const m_iShardCount: usize = 0x2F0; //
        pub const m_vLastShardPosition: usize = 0x2F4; //
        pub const m_hSurfShard: usize = 0x300; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_ChronoSwap {
        pub const m_bHitTarget: usize = 0xCA0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CAbilityCardTossVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SummonedCard: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strCardTossSound: usize = 0x1750; //
        pub const m_strCardSummonSound: usize = 0x1760; //
        pub const m_flSummonedCardStartSideOffset: usize = 0x1770; //
        pub const m_flSummonedCardSideOffsetStep: usize = 0x1774; //
        pub const m_flSummonedCardForwardOffset: usize = 0x1778; //
        pub const m_flSummonedCardVerticalOffset: usize = 0x177C; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_TriggerTower {
    }
    // Parent: C_PhysicsProp
    // Field count: 0
    pub mod C_ItemParachute {
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_FuncRotating {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_Tornado_Aura_Apply {
    }
    // Parent: CCitadel_Modifier_Base
    // Field count: 0
    pub mod CModifier_Synth_Blitz_TechAmp {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_SpreadingFire_DOT_VData {
        pub const m_sSpreadingFireParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sSpreadingFireTetherParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierFlyingStrikeTargetVData {
        pub const m_GrappleRopeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 15
    pub mod CCitadel_Modifier_Nano_PredatoryStatueVData {
        pub const m_AOEParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnabledParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DrainParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strEnabledSound: usize = 0x8D8; //
        pub const m_strEnabledLoopSound: usize = 0x8E8; //
        pub const m_strDisabledSound: usize = 0x8F8; //
        pub const m_strLaserHitSound: usize = 0x908; //
        pub const m_strLaserStartSound: usize = 0x918; //
        pub const m_strLaserLoopSound: usize = 0x928; //
        pub const m_TargetModifier: usize = 0x938; //
        pub const m_RevealModifier: usize = 0x948; //
        pub const m_StatueInvis: usize = 0x958; //
        pub const m_flNewTargetAttackTime: usize = 0x968; //
        pub const m_flMinRevealTime: usize = 0x96C; //
        pub const m_flMinDebuffTime: usize = 0x970; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierStackingDamageVData {
        pub const m_SlowModifier: usize = 0x638; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_BloodBomb {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierStimPakVData {
        pub const m_BuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_MagicShock_Proc {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 6
    pub mod CCitadel_Ability_Sprint {
        pub const m_nSprintParticle: usize = 0xCA0; //
        pub const m_bSprinting: usize = 0xCA4; //
        pub const m_flInCombatStartTime: usize = 0xCA8; //
        pub const m_flInCombatEndTime: usize = 0xCAC; //
        pub const m_flSprintStartTime: usize = 0xCB0; //
        pub const m_bInCombat: usize = 0xCB4; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_DamageResistance {
        pub const m_flShieldHealth: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Healing_Disabled {
    }
    // Parent: C_BaseEntity
    // Field count: 4
    pub mod C_BaseFire {
        pub const m_flScale: usize = 0x560; //
        pub const m_flStartScale: usize = 0x564; //
        pub const m_flScaleTime: usize = 0x568; //
        pub const m_nFlags: usize = 0x56C; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_DebuffReducer {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Empty {
    }
    // Parent: C_CitadelTrackedProjectile
    // Field count: 1
    pub mod C_CitadelPositionHomingProjectile {
        pub const m_vecHomingPosition: usize = 0x8E0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CGameModifier_FireUserEntityIO {
    }
    // Parent: C_BaseEntity
    // Field count: 16
    pub mod CBasePlayerController {
        pub const m_nFinalPredictedTick: usize = 0x568; //
        pub const m_CommandContext: usize = 0x570; //
        pub const m_nInButtonsWhichAreToggles: usize = 0x628; //
        pub const m_nTickBase: usize = 0x630; //
        pub const m_hPawn: usize = 0x634; //
        pub const m_bKnownTeamMismatch: usize = 0x638; //
        pub const m_hPredictedPawn: usize = 0x63C; //
        pub const m_nSplitScreenSlot: usize = 0x640; //
        pub const m_hSplitOwner: usize = 0x644; //
        pub const m_hSplitScreenPlayers: usize = 0x648; //
        pub const m_bIsHLTV: usize = 0x660; //
        pub const m_iConnected: usize = 0x664; //
        pub const m_iszPlayerName: usize = 0x668; // char[128]
        pub const m_steamID: usize = 0x6F0; //
        pub const m_bIsLocalPlayerController: usize = 0x6F8; //
        pub const m_iDesiredFOV: usize = 0x6FC; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_GenericPerson_3 {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_HealthSwapVData {
        pub const m_BloodExchangeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Muted {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Silenced {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_TechBleed_Proc {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_ApplyDebuff_ProcVData {
        pub const m_bUseNonEmbedded: usize = 0x668; //
        pub const m_DebuffModifier: usize = 0x670; //
        pub const m_NonEmbeddedModifier: usize = 0x680; //
    }
    // Parent: CBaseModifier
    // Field count: 0
    pub mod CCitadelModifier {
    }
    // Parent: None
    // Field count: 0
    pub mod CPointTemplateAPI {
    }
    // Parent: CBaseAnimGraph
    // Field count: 0
    pub mod C_WaterBullet {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Synth_Affliction_Debuff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 6
    pub mod CModifierGangActivityAbilitySwapVData {
        pub const m_SummonGangster: usize = 0x638; //
        pub const m_TeleportToGangster: usize = 0x648; //
        pub const m_Cancel: usize = 0x658; //
        pub const m_ReplaceWithSummonGangster: usize = 0x668; //
        pub const m_ReplaceWithTeleportToGangster: usize = 0x678; //
        pub const m_ReplaceWithCancel: usize = 0x688; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 10
    pub mod CModifierRestorativeGooVData {
        pub const m_RestorativeGooEndParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flDistanceCameraOffsetLerpTime: usize = 0x718; //
        pub const m_flDistanceCameraOffsetBias: usize = 0x71C; //
        pub const m_flDistanceCameraOffset: usize = 0x720; //
        pub const m_BreakoutProgressBarModifier: usize = 0x728; //
        pub const m_PostCubeBuffModifier: usize = 0x738; //
        pub const m_NonTargetLoopingSound: usize = 0x748; //
        pub const m_TargetLoopingSound: usize = 0x758; //
        pub const m_LightMeleeImpact: usize = 0x768; //
        pub const m_HeavyMeleeImpact: usize = 0x778; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Chrono_PulseGrenade_Debuff {
    }
    // Parent: CCitadelModifier
    // Field count: 5
    pub mod CCitadel_Modifier_BeltFed_Magazine {
        pub const m_bInitialized: usize = 0xC0; //
        pub const m_flSpinUpRateOverride: usize = 0xC4; //
        pub const m_flSpinUpDecayOverride: usize = 0xC8; //
        pub const m_flMaxCycleTimeOverride: usize = 0xCC; //
        pub const m_flMaxBurstFireCooldownOverride: usize = 0xD0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ItemWalkBack {
    }
    // Parent: CSkeletonAnimationController
    // Field count: 14
    pub mod CBaseAnimGraphController {
        pub const m_animGraphNetworkedVars: usize = 0x18; //
        pub const m_bSequenceFinished: usize = 0x14A8; //
        pub const m_flSoundSyncTime: usize = 0x14AC; //
        pub const m_nActiveIKChainMask: usize = 0x14B0; //
        pub const m_hSequence: usize = 0x14B4; //
        pub const m_flSeqStartTime: usize = 0x14B8; //
        pub const m_flSeqFixedCycle: usize = 0x14BC; //
        pub const m_nAnimLoopMode: usize = 0x14C0; //
        pub const m_flPlaybackRate: usize = 0x14C4; //
        pub const m_nNotifyState: usize = 0x14D0; //
        pub const m_bNetworkedAnimationInputsChanged: usize = 0x14D2; //
        pub const m_bNetworkedSequenceChanged: usize = 0x14D3; //
        pub const m_bLastUpdateSkipped: usize = 0x14D4; //
        pub const m_flPrevAnimUpdateTime: usize = 0x14D8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_HealthSwap {
        pub const m_nFXIndex: usize = 0xCA0; //
        pub const m_flHealthToCaster: usize = 0xCA4; //
        pub const m_flTargetHealthLost: usize = 0xCA8; //
        pub const m_flPostCastHoldEndTime: usize = 0xE38; //
    }
    // Parent: CCitadel_Modifier_ChainLightningVData
    // Field count: 1
    pub mod CCitadel_Modifier_Galvanic_Storm_VData {
        pub const m_TechShieldModifier: usize = 0x858; //
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_Push {
        pub const m_vPushForce: usize = 0xC0; //
        pub const m_flDecayRate: usize = 0xCC; //
        pub const m_TimeDestroy: usize = 0xD0; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Hero_Testing_Damage_Aura {
    }
    // Parent: CModifierVData_BaseAura
    // Field count: 3
    pub mod CCitadelModifierAuraVData {
        pub const m_iAuraSearchType: usize = 0x670; //
        pub const m_iAuraSearchFlags: usize = 0x674; //
        pub const m_eLosCheck: usize = 0x678; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CAbility_Synth_Barrage_VData {
        pub const m_BarrageCasterModifier: usize = 0x1590; //
        pub const m_AmpModifier: usize = 0x15A0; //
        pub const m_DebuffModifier: usize = 0x15B0; //
        pub const m_ShootParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x16A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChannelParticle: usize = 0x1780; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strProjectileLaunchSound: usize = 0x1860; //
        pub const m_flAttackInterval: usize = 0x1870; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CModifierUppercuttedVData {
        pub const m_StunParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strStunSound: usize = 0x718; //
        pub const m_NoExplodeModifier: usize = 0x728; //
        pub const m_ExplodeDebuffModifier: usize = 0x738; //
        pub const m_flEnemyNoAirDashDuration: usize = 0x748; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_PsychicDagger {
    }
    // Parent: C_BaseModelEntity
    // Field count: 15
    pub mod CBaseAnimGraph {
        pub const m_bInitiallyPopulateInterpHistory: usize = 0x8D8; //
        pub const m_bSuppressAnimEventSounds: usize = 0x8DA; //
        pub const m_bAnimGraphUpdateEnabled: usize = 0x8E8; //
        pub const m_flMaxSlopeDistance: usize = 0x8EC; //
        pub const m_vLastSlopeCheckPos: usize = 0x8F0; //
        pub const m_bAnimationUpdateScheduled: usize = 0x8FC; //
        pub const m_vecForce: usize = 0x900; //
        pub const m_nForceBone: usize = 0x90C; //
        pub const m_pClientsideRagdoll: usize = 0x910; //
        pub const m_bBuiltRagdoll: usize = 0x918; //
        pub const m_RagdollPose: usize = 0x930; //
        pub const m_bRagdollClientSide: usize = 0x978; //
        pub const m_bHasAnimatedMaterialAttributes: usize = 0x988; //
        pub const m_animGraph2SerializeData: usize = 0xAD0; //
        pub const m_nAnimGraph2SerializeDataSizeBytes: usize = 0xAE8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Tokamak_HeatSinks_DOT_VData {
        pub const m_sAfterburnParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sAfterburnExplodeParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CAbilityMeleeVData
    // Field count: 6
    pub mod CAbilityUppercutVData {
        pub const m_UppercutAttackData: usize = 0x15B8; //
        pub const m_UppercutModifier: usize = 0x1AE0; //
        pub const m_BuffModifier: usize = 0x1AF0; //
        pub const m_ClipModifier: usize = 0x1B00; //
        pub const m_flMaxPitchUp: usize = 0x1B10; //
        pub const m_flTossDuration: usize = 0x1B14; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Gravity_Lasso {
    }
    // Parent: CCitadelModifierVData
    // Field count: 6
    pub mod CCitadel_Modifier_IceDomeVData {
        pub const m_BlockerModel: usize = 0x638; //
        pub const m_DomeParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FriendlyAuraModifier: usize = 0x7F8; //
        pub const m_EnemyAuraModifier: usize = 0x808; //
        pub const m_strDomeEndSound: usize = 0x818; //
        pub const m_strTargetLoopingSound: usize = 0x828; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_HealthSwapPrecast {
        pub const m_hTarget: usize = 0xC0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_StaticChargeVData {
        pub const m_CastParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StaticChargeModifier: usize = 0x1670; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_MeleeDamageOnly {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CModifier_Mirage_Tornado_Aura {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_MobileResupplyAura {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Citadel_Projectile_Guided_Arrow {
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod C_PointEntity {
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 4
    pub mod CCitadel_Modifier_PetrifyVData {
        pub const m_DebuffParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffStartParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffEndParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PostSleepModifier: usize = 0x9B8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_IceDomeVData {
        pub const m_IceDomeModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierLashFlogDebuffVData {
        pub const m_FlogDebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_GameRules
    // Field count: 0
    pub mod C_MultiplayRules {
    }
    // Parent: None
    // Field count: 0
    pub mod CBasePlayerControllerAPI {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_ControlPointBlockerAura {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_Item_WarpStone {
        pub const m_nCastDelayParticleIndex: usize = 0xCB8; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_SiphonBullets {
        pub const m_iStacks: usize = 0xCB8; //
    }
    // Parent: C_BaseEntity
    // Field count: 16
    pub mod C_PathParticleRope {
        pub const m_bStartActive: usize = 0x568; //
        pub const m_flMaxSimulationTime: usize = 0x56C; //
        pub const m_iszEffectName: usize = 0x570; //
        pub const m_PathNodes_Name: usize = 0x578; //
        pub const m_flParticleSpacing: usize = 0x590; //
        pub const m_flSlack: usize = 0x594; //
        pub const m_flRadius: usize = 0x598; //
        pub const m_ColorTint: usize = 0x59C; //
        pub const m_nEffectState: usize = 0x5A0; //
        pub const m_iEffectIndex: usize = 0x5A8; //
        pub const m_PathNodes_Position: usize = 0x5B0; //
        pub const m_PathNodes_TangentIn: usize = 0x5C8; //
        pub const m_PathNodes_TangentOut: usize = 0x5E0; //
        pub const m_PathNodes_Color: usize = 0x5F8; //
        pub const m_PathNodes_PinEnabled: usize = 0x610; //
        pub const m_PathNodes_RadiusScale: usize = 0x628; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_RapidFire {
        pub const m_flNextAttackTime: usize = 0x210; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ThrownShiv_Slow_Debuff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_SilencedVData {
        pub const m_EmpParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EmpPlayerParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EmpStatusParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_BulletShieldImpact {
        pub const m_AmbientEffect: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_PullDownToGroundVData {
        pub const m_flMaxHeight: usize = 0x638; //
        pub const m_flPullDownSpeedMin: usize = 0x63C; //
        pub const m_flPullDownSpeedScale: usize = 0x640; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Backdoor_Protection {
    }
    // Parent: CAI_CitadelNPCVData
    // Field count: 32
    pub mod CNPC_TrooperNeutralVData {
        pub const m_eTrooperType: usize = 0x10E8; //
        pub const m_flGoldReward: usize = 0x10EC; //
        pub const m_flGoldRewardBonusPercentPerMinute: usize = 0x10F0; //
        pub const m_bGiveGoldOnHit: usize = 0x10F4; //
        pub const m_bOrbDropper: usize = 0x10F5; //
        pub const m_bCapSimultanousAttackers: usize = 0x10F6; //
        pub const m_flShieldReactivateDelay: usize = 0x10F8; //
        pub const m_flDyingDuration: usize = 0x10FC; //
        pub const m_bDamagedByBullets: usize = 0x1100; //
        pub const m_bDamagedByMelee: usize = 0x1101; //
        pub const m_bDamagedByAbilities: usize = 0x1102; //
        pub const m_bFixedMeleeDamage: usize = 0x1103; //
        pub const m_ShieldParticle: usize = 0x1108; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flRetaliateDamage: usize = 0x11E8; //
        pub const m_flRetaliateCooldown: usize = 0x11EC; //
        pub const m_retaliateParticle: usize = 0x11F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_bHasAOEAttack: usize = 0x12D0; //
        pub const m_flAOERadius: usize = 0x12D4; //
        pub const m_flAOEDamage: usize = 0x12D8; //
        pub const m_flAOEAttackCooldown: usize = 0x12DC; //
        pub const m_AOEParticle: usize = 0x12E0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AOEDebuffToApply: usize = 0x13C0; //
        pub const m_AOEInitiateSound: usize = 0x13D0; //
        pub const m_AOESound: usize = 0x13E0; //
        pub const m_AOEDebuffDuration: usize = 0x13F0; //
        pub const m_vecRandomBodyGroup: usize = 0x13F8; //
        pub const m_vecRandomSkin: usize = 0x1410; //
        pub const m_flHullCapsuleRadius: usize = 0x1428; //
        pub const m_flHullCapsuleHeight: usize = 0x142C; //
        pub const m_bFaceEnemyWhileIdle: usize = 0x1430; //
        pub const m_IdleLoopSound: usize = 0x1438; //
        pub const m_MoveType: usize = 0x1448; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod CCitadel_Projectile_RocketLauncher_Rocket {
    }
    // Parent: CCitadel_Ability_BaseHeldItem
    // Field count: 0
    pub mod CCitadel_Ability_GoldenIdol {
    }
    // Parent: C_BaseModelEntity
    // Field count: 16
    pub mod C_CitadelProjectile {
        pub const m_flMaxDistance: usize = 0x864; //
        pub const m_flArmingTime: usize = 0x868; //
        pub const m_flChargeAmount: usize = 0x86C; //
        pub const m_bCollideWithThrower: usize = 0x870; //
        pub const m_bNewCollideWithThrower: usize = 0x871; //
        pub const m_flTickSoundInterval: usize = 0x880; //
        pub const m_vInitialVelocity: usize = 0x888; //
        pub const m_vInitialPosition: usize = 0x894; //
        pub const m_abilityID: usize = 0x8A0; //
        pub const m_hThrower: usize = 0x8A4; //
        pub const m_sParticleName: usize = 0x8A8; //
        pub const m_vecSpawnPosition: usize = 0x8B0; //
        pub const m_flProjectileSpeed: usize = 0x8BC; //
        pub const m_flMaxLifetime: usize = 0x8C0; //
        pub const m_flParticleRadius: usize = 0x8C8; //
        pub const m_flPreviousTimeScale: usize = 0x8D8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Targetdummy_3 {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityVandalSurgeVData {
        pub const m_LiftModifier: usize = 0x1590; //
        pub const m_TargetParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetCastSound: usize = 0x1680; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_StaticCharge {
        pub const m_hRingEffect: usize = 0xC0; //
        pub const m_flRadius: usize = 0x138; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Upgrade_KineticSash {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_CloakingDevice {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_RegenerativeArmor {
    }
    // Parent: C_SoundEventEntity
    // Field count: 2
    pub mod C_SoundEventAABBEntity {
        pub const m_vMins: usize = 0x620; //
        pub const m_vMaxs: usize = 0x62C; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CCitadel_Ability_Fathom_Breach_VData {
        pub const m_ExplosionParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LeapParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strInFlightAnimGraphParam: usize = 0x1750; //
        pub const m_strExplodeSound: usize = 0x1758; //
        pub const m_InFlightModifier: usize = 0x1768; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Citadel_Bull_Leap_LandingBonuses_VData {
        pub const m_BuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 23
    pub mod CAbilityDashVData {
        pub const m_DashParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DownDashParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strArriveSound: usize = 0x1750; //
        pub const m_strStaminaDrainedSound: usize = 0x1760; //
        pub const m_cameraSequenceGroundDashActivate: usize = 0x1770; //
        pub const m_cameraSequenceAirDashActivate: usize = 0x17F8; //
        pub const m_flMaxAngDiff: usize = 0x1880; //
        pub const m_flDurationScaleForSpeed: usize = 0x1884; //
        pub const m_flSlideEarlyOutWindow: usize = 0x1888; //
        pub const m_flSlideLockoutTime: usize = 0x188C; //
        pub const m_flGroundDashAirbornDrag: usize = 0x1890; //
        pub const m_flGroundDashAirbornSpeedClamp: usize = 0x1894; //
        pub const m_strGroundDashActivate: usize = 0x1898; //
        pub const m_curvePosition: usize = 0x18A8; //
        pub const m_flGroundDashDuration: usize = 0x18E8; //
        pub const m_flGroundDashDistanceInMeters: usize = 0x18EC; //
        pub const m_flAirDashEndVelocityScale: usize = 0x18F0; //
        pub const m_flAirDashAccPct: usize = 0x18F4; //
        pub const m_flDuringDrag: usize = 0x18F8; //
        pub const m_flPostDrag: usize = 0x18FC; //
        pub const m_flPostDragDuration: usize = 0x1900; //
        pub const m_flDownwardAirDashSpeed: usize = 0x1904; //
        pub const m_strDashActivate: usize = 0x1908; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_DebuffImmunity {
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 1
    pub mod CCitadel_Modifier_Wrecker_Ultimate_ThrowEnemy {
        pub const m_vThrowVelocity: usize = 0xC8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_WreckerSalvage_Buff {
        pub const m_nBuffParticle: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_StickyBombAttached {
        pub const m_bDetonateSoundStarted: usize = 0xC0; //
        pub const m_nParticleIndex: usize = 0xCC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_ViscousBallVData {
        pub const m_TrailParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DirectionParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityNikumanVData {
        pub const m_NikumanModifier: usize = 0x1590; //
        pub const m_SelfBuffModifier: usize = 0x15A0; //
    }
    // Parent: CitadelItemVData
    // Field count: 6
    pub mod CItem_ActiveReload_VData {
        pub const m_SuccessModifier: usize = 0x15D8; //
        pub const m_strSuccessSound: usize = 0x15E8; //
        pub const m_strFailureSound: usize = 0x15F8; //
        pub const m_SuccessParticle: usize = 0x1608; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FailureParticle: usize = 0x16E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flGraceTime: usize = 0x17C8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 10
    pub mod CitadelItemVData {
        pub const m_iItemTier: usize = 0x1594; // EModTier_t
        pub const m_nUpgradeSlotCost: usize = 0x1595; //
        pub const m_bWarnIfNoAffectedAbilities: usize = 0x1596; //
        pub const m_bRequiresChargedAbility: usize = 0x1597; //
        pub const m_bRequiresChanelledAbility: usize = 0x1598; //
        pub const m_vecComponentItems: usize = 0x15A0; //
        pub const m_bShowTextDescription: usize = 0x15B8; //
        pub const m_bIsDefensiveItem: usize = 0x15B9; //
        pub const m_eShopFilters: usize = 0x15BA; //
        pub const m_vecTooltipSectionInfo: usize = 0x15C0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Near_Climbable_RopeVData {
        pub const m_flEnableStateTime: usize = 0x638; //
    }
    // Parent: CEnvSoundscape
    // Field count: 0
    pub mod CEnvSoundscapeAlias_snd_soundscape {
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 2
    pub mod CCitadel_Modifier_FlameDashGroundAuraVData {
        pub const m_GroundParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flHeight: usize = 0x760; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_QuickSilver {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_AOE_Tech_Shield {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CGameModifier_SetMoveType {
        pub const m_nMoveType: usize = 0xC0; //
    }
    // Parent: CCitadel_Modifier_Base
    // Field count: 0
    pub mod CCitadel_Modifier_TangoTetherTarget {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PristineEmblem {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Objective_Bullet_Resist {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Tokamak_AllySmokeAOE {
    }
    // Parent: CCitadel_Item_TrackingProjectileApplyModifier
    // Field count: 0
    pub mod CCitadel_Item_Disarm {
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CModifier_Synth_Barrage_Caster_VData {
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityGangActivityCancelVData {
        pub const m_AbilitySwap: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 5
    pub mod CCitadel_Modifier_HookTarget {
        pub const m_flCurrentVerticalSpeed: usize = 0xC4; //
        pub const m_bSuccess: usize = 0xC8; //
        pub const m_bSameTeam: usize = 0xC9; //
        pub const m_bPlayedApproachingWhoosh: usize = 0xCA; //
        pub const m_flInitialTravelDistance: usize = 0xCC; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilitySleepDaggerVData {
        pub const m_ImpactParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SleepModifier: usize = 0x1670; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbilityLashFlogVData {
        pub const m_FlogParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FlogLifeLeachParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strHitConfirmSound: usize = 0x1750; //
        pub const m_FlogDebuffModifier: usize = 0x1760; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_BoxingGlove {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_Inhibitor_ProcVData {
        pub const m_DebuffModifier: usize = 0x668; //
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProc
    // Field count: 1
    pub mod CCitadel_Modifier_ChainLightning {
        pub const m_flNextProcTime: usize = 0x188; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 7
    pub mod C_BasePlayerWeapon {
        pub const m_nNextPrimaryAttackTick: usize = 0xAF8; //
        pub const m_flNextPrimaryAttackTickRatio: usize = 0xAFC; //
        pub const m_nNextSecondaryAttackTick: usize = 0xB00; //
        pub const m_flNextSecondaryAttackTickRatio: usize = 0xB04; //
        pub const m_iClip1: usize = 0xB08; //
        pub const m_iClip2: usize = 0xB0C; //
        pub const m_pReserveAmmo: usize = 0xB10; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_UtilityUpgrade_HealthNova {
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CModifierSpiderShieldBuffVData {
        pub const m_BuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RadiusParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PulseParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PulseDebuffModifier: usize = 0x8D8; //
    }
    // Parent: CCitadelYamatoBaseVData
    // Field count: 9
    pub mod CCitadelAbilityHealingSlashVData {
        pub const m_flEffectSize: usize = 0x1598; //
        pub const m_flMaxAttackAngle: usize = 0x159C; //
        pub const m_remapAngleToTime: usize = 0x15A0; //
        pub const m_DebuffModifier: usize = 0x15B0; //
        pub const m_ImpactParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HealingSlashParticle: usize = 0x16A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HealingSlashSwordGlow: usize = 0x1780; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1860; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDamageTarget: usize = 0x1940; //
    }
    // Parent: None
    // Field count: 24
    pub mod CBasePlayerWeaponVData {
        pub const m_szClassName: usize = 0x10; //
        pub const m_szWorldModel: usize = 0x18; //
        pub const m_sToolsOnlyOwnerModelName: usize = 0xF8; //
        pub const m_bBuiltRightHanded: usize = 0x1D8; //
        pub const m_bAllowFlipping: usize = 0x1D9; //
        pub const m_sMuzzleAttachment: usize = 0x1E0; //
        pub const m_szMuzzleFlashParticle: usize = 0x200; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_bLinkedCooldowns: usize = 0x2E0; //
        pub const m_vecIntrinsicModifiers: usize = 0x2E8; //
        pub const m_iFlags: usize = 0x300; //
        pub const m_nPrimaryAmmoType: usize = 0x301; //
        pub const m_nSecondaryAmmoType: usize = 0x302; //
        pub const m_iMaxClip1: usize = 0x304; //
        pub const m_iMaxClip2: usize = 0x308; //
        pub const m_iDefaultClip1: usize = 0x30C; //
        pub const m_iDefaultClip2: usize = 0x310; //
        pub const m_bReserveAmmoAsClips: usize = 0x314; //
        pub const m_iWeight: usize = 0x318; //
        pub const m_bAutoSwitchTo: usize = 0x31C; //
        pub const m_bAutoSwitchFrom: usize = 0x31D; //
        pub const m_iRumbleEffect: usize = 0x320; //
        pub const m_iSlot: usize = 0x324; //
        pub const m_iPosition: usize = 0x328; //
        pub const m_aShootSounds: usize = 0x330; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 23
    pub mod C_Fish {
        pub const m_pos: usize = 0xAF8; //
        pub const m_vel: usize = 0xB04; //
        pub const m_angles: usize = 0xB10; //
        pub const m_localLifeState: usize = 0xB1C; //
        pub const m_deathDepth: usize = 0xB20; //
        pub const m_deathAngle: usize = 0xB24; //
        pub const m_buoyancy: usize = 0xB28; //
        pub const m_wiggleTimer: usize = 0xB30; //
        pub const m_wigglePhase: usize = 0xB48; //
        pub const m_wiggleRate: usize = 0xB4C; //
        pub const m_actualPos: usize = 0xB50; //
        pub const m_actualAngles: usize = 0xB5C; //
        pub const m_poolOrigin: usize = 0xB68; //
        pub const m_waterLevel: usize = 0xB74; //
        pub const m_gotUpdate: usize = 0xB78; //
        pub const m_x: usize = 0xB7C; //
        pub const m_y: usize = 0xB80; //
        pub const m_z: usize = 0xB84; //
        pub const m_angle: usize = 0xB88; //
        pub const m_errorHistory: usize = 0xB8C; //
        pub const m_errorHistoryIndex: usize = 0xBDC; //
        pub const m_errorHistoryCount: usize = 0xBE0; //
        pub const m_averageError: usize = 0xBE4; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Citadel_Projectile_Archer_ChargedShot {
    }
    // Parent: CCitadelModifierAura
    // Field count: 3
    pub mod CCitadel_Modifier_Item_AOESilence {
        pub const m_flStartRadius: usize = 0xF0; //
        pub const m_flEndRadius: usize = 0xF4; //
        pub const m_flSpreadDuration: usize = 0xF8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CCitadel_Ability_Trapper_WebWallVData {
        pub const m_DebuffModifier: usize = 0x1590; //
        pub const m_SilenceModifier: usize = 0x15A0; //
        pub const m_WebWallParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WebWallDestroyedParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TimeWallChargeParticle: usize = 0x1770; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TimeWallHitParticle: usize = 0x1850; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TimeWallHitTimerParticle: usize = 0x1930; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strWebWallCreated: usize = 0x1A10; //
        pub const m_strWebWallDestroyed: usize = 0x1A20; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Shiv_KillingBlow_Leap {
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbilityLightningBallVData {
        pub const m_ZapModifier: usize = 0x1590; //
        pub const m_strHitSound: usize = 0x15A0; //
        pub const m_strProjectileLoopingSound: usize = 0x15B0; //
        pub const m_ZapParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CScaleFunctionVData
    // Field count: 0
    pub mod CScaleFunctionAbilityPropertySingleStatVData {
    }
    // Parent: IntervalTimer
    // Field count: 7
    pub mod CTimeline {
        pub const m_flValues: usize = 0x10; //
        pub const m_nValueCounts: usize = 0x110; //
        pub const m_nBucketCount: usize = 0x210; //
        pub const m_flInterval: usize = 0x214; //
        pub const m_flFinalValue: usize = 0x218; //
        pub const m_nCompressionType: usize = 0x21C; //
        pub const m_bStopped: usize = 0x220; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 0
    pub mod C_Citadel_DeployablePreview {
    }
    // Parent: CCitadelModifierAura
    // Field count: 1
    pub mod CCitadel_Item_StasisBomb_Aura {
        pub const m_AuraRadius: usize = 0xF0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Targetdummy_1 {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SettingSunThinker {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierHighAlertBuffVData {
        pub const m_BuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityTrappersBoloVData {
        pub const m_ImpactParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TrapModifier: usize = 0x1670; //
        pub const m_DebuffModifier: usize = 0x1680; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_VandalSurge {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_DisarmProcWatcher {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_RevealTarget {
    }
    // Parent: C_BaseEntity
    // Field count: 2
    pub mod CPathAccompany {
        pub const m_flPathLength: usize = 0x560; //
        pub const m_vecNodes: usize = 0x568; //
    }
    // Parent: C_BaseCombatCharacter
    // Field count: 28
    pub mod C_BasePlayerPawn {
        pub const m_pWeaponServices: usize = 0xD18; //
        pub const m_pItemServices: usize = 0xD20; //
        pub const m_pAutoaimServices: usize = 0xD28; //
        pub const m_pObserverServices: usize = 0xD30; //
        pub const m_pWaterServices: usize = 0xD38; //
        pub const m_pUseServices: usize = 0xD40; //
        pub const m_pFlashlightServices: usize = 0xD48; //
        pub const m_pCameraServices: usize = 0xD50; //
        pub const m_pMovementServices: usize = 0xD58; //
        pub const m_ServerViewAngleChanges: usize = 0xD68; // C_UtlVectorEmbeddedNetworkVar<ViewAngleServerChange_t>
        pub const v_angle: usize = 0xDD0; //
        pub const v_anglePrevious: usize = 0xDDC; //
        pub const m_iHideHUD: usize = 0xDE8; //
        pub const m_skybox3d: usize = 0xDF0; //
        pub const m_flDeathTime: usize = 0xE80; //
        pub const m_vecPredictionError: usize = 0xE84; //
        pub const m_flPredictionErrorTime: usize = 0xE90; //
        pub const m_vecLastCameraSetupLocalOrigin: usize = 0xE94; //
        pub const m_flLastCameraSetupTime: usize = 0xEA0; //
        pub const m_flFOVSensitivityAdjust: usize = 0xEA4; //
        pub const m_flMouseSensitivity: usize = 0xEA8; //
        pub const m_vOldOrigin: usize = 0xEAC; //
        pub const m_flOldSimulationTime: usize = 0xEB8; //
        pub const m_nLastExecutedCommandNumber: usize = 0xEBC; //
        pub const m_nLastExecutedCommandTick: usize = 0xEC0; //
        pub const m_hController: usize = 0xEC4; //
        pub const m_hDefaultController: usize = 0xEC8; //
        pub const m_bIsSwappingToPredictableController: usize = 0xECC; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_TimeWall_AuraVData {
        pub const m_DebuffModifier: usize = 0x680; //
    }
    // Parent: None
    // Field count: 7
    pub mod CAttributeManager {
        pub const m_Providers: usize = 0x8; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_Receivers: usize = 0x20; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_iReapplyProvisionParity: usize = 0x38; //
        pub const m_hOuter: usize = 0x3C; //
        pub const m_bPreventLoopback: usize = 0x40; //
        pub const m_ProviderType: usize = 0x44; //
        pub const m_CachedResults: usize = 0x48; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_Magician_AnimalCurse {
        pub const m_CachedTarget: usize = 0xCA0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityDistruptiveChargeVData {
        pub const m_Particle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffModifier: usize = 0x1670; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_HornetSting {
        pub const m_BounceCount: usize = 0xCA0; //
        pub const m_bHitHero: usize = 0xCA4; //
        pub const m_vecValidBounceTargets: usize = 0xCA8; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_MutedVData {
        pub const m_MutedParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_MutedPlayerParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_MutedStatusParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_TurnCameraToTarget {
        pub const m_hTarget: usize = 0xC0; //
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod CLogicalEntity {
    }
    // Parent: None
    // Field count: 24
    pub mod CAnimGraphNetworkedVariables {
        pub const m_PredNetBoolVariables: usize = 0x8; //
        pub const m_PredNetByteVariables: usize = 0x20; //
        pub const m_PredNetUInt16Variables: usize = 0x38; //
        pub const m_PredNetIntVariables: usize = 0x50; //
        pub const m_PredNetUInt32Variables: usize = 0x68; //
        pub const m_PredNetUInt64Variables: usize = 0x80; //
        pub const m_PredNetFloatVariables: usize = 0x98; //
        pub const m_PredNetVectorVariables: usize = 0xB0; //
        pub const m_PredNetQuaternionVariables: usize = 0xC8; //
        pub const m_PredNetGlobalSymbolVariables: usize = 0xE0; //
        pub const m_OwnerOnlyPredNetBoolVariables: usize = 0xF8; //
        pub const m_OwnerOnlyPredNetByteVariables: usize = 0x110; //
        pub const m_OwnerOnlyPredNetUInt16Variables: usize = 0x128; //
        pub const m_OwnerOnlyPredNetIntVariables: usize = 0x140; //
        pub const m_OwnerOnlyPredNetUInt32Variables: usize = 0x158; //
        pub const m_OwnerOnlyPredNetUInt64Variables: usize = 0x170; //
        pub const m_OwnerOnlyPredNetFloatVariables: usize = 0x188; //
        pub const m_OwnerOnlyPredNetVectorVariables: usize = 0x1A0; //
        pub const m_OwnerOnlyPredNetQuaternionVariables: usize = 0x1B8; //
        pub const m_OwnerOnlyPredNetGlobalSymbolVariables: usize = 0x1D0; //
        pub const m_nBoolVariablesCount: usize = 0x1E8; //
        pub const m_nOwnerOnlyBoolVariablesCount: usize = 0x1EC; //
        pub const m_nRandomSeedOffset: usize = 0x1F0; //
        pub const m_flLastTeleportTime: usize = 0x1F4; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 41
    pub mod C_RopeKeyframe {
        pub const m_LinksTouchingSomething: usize = 0x860; //
        pub const m_nLinksTouchingSomething: usize = 0x864; //
        pub const m_bApplyWind: usize = 0x868; //
        pub const m_fPrevLockedPoints: usize = 0x86C; //
        pub const m_iForcePointMoveCounter: usize = 0x870; //
        pub const m_bPrevEndPointPos: usize = 0x874; //
        pub const m_vPrevEndPointPos: usize = 0x878; //
        pub const m_flCurScroll: usize = 0x890; //
        pub const m_flScrollSpeed: usize = 0x894; //
        pub const m_RopeFlags: usize = 0x898; //
        pub const m_iRopeMaterialModelIndex: usize = 0x8A0; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_LightValues: usize = 0xB18; //
        pub const m_nSegments: usize = 0xB90; //
        pub const m_hStartPoint: usize = 0xB94; //
        pub const m_hEndPoint: usize = 0xB98; //
        pub const m_iStartAttachment: usize = 0xB9C; //
        pub const m_iEndAttachment: usize = 0xB9D; //
        pub const m_Subdiv: usize = 0xB9E; //
        pub const m_RopeLength: usize = 0xBA0; //
        pub const m_Slack: usize = 0xBA2; //
        pub const m_TextureScale: usize = 0xBA4; //
        pub const m_fLockedPoints: usize = 0xBA8; //
        pub const m_nChangeCount: usize = 0xBA9; //
        pub const m_Width: usize = 0xBAC; //
        pub const m_PhysicsDelegate: usize = 0xBB0; // C_RopeKeyframe::CPhysicsDelegate
        pub const m_hMaterial: usize = 0xBC0; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_TextureHeight: usize = 0xBC8; //
        pub const m_vecImpulse: usize = 0xBCC; //
        pub const m_vecPreviousImpulse: usize = 0xBD8; //
        pub const m_flCurrentGustTimer: usize = 0xBE4; //
        pub const m_flCurrentGustLifetime: usize = 0xBE8; //
        pub const m_flTimeToNextGust: usize = 0xBEC; //
        pub const m_vWindDir: usize = 0xBF0; //
        pub const m_vColorMod: usize = 0xBFC; //
        pub const m_vCachedEndPointAttachmentPos: usize = 0xC08; //
        pub const m_vCachedEndPointAttachmentAngle: usize = 0xC20; //
        pub const m_bConstrainBetweenEndpoints: usize = 0xC38; //
        pub const m_bEndPointAttachmentPositionsDirty: usize = 0x0; //
        pub const m_bEndPointAttachmentAnglesDirty: usize = 0x0; //
        pub const m_bNewDataThisFrame: usize = 0x0; //
        pub const m_bPhysicsInitted: usize = 0x0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_SilenceContraptionsDebuffVData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Trappers_Bolo {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Nano_PredatoryStatueTarget {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_SlowingBullets_Proc {
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod CFilterModifier {
        pub const m_iFilterModifier: usize = 0x5B8; //
    }
    // Parent: C_BaseEntity
    // Field count: 16
    pub mod C_GradientFog {
        pub const m_hGradientFogTexture: usize = 0x560; //
        pub const m_flFogStartDistance: usize = 0x568; //
        pub const m_flFogEndDistance: usize = 0x56C; //
        pub const m_bHeightFogEnabled: usize = 0x570; //
        pub const m_flFogStartHeight: usize = 0x574; //
        pub const m_flFogEndHeight: usize = 0x578; //
        pub const m_flFarZ: usize = 0x57C; //
        pub const m_flFogMaxOpacity: usize = 0x580; //
        pub const m_flFogFalloffExponent: usize = 0x584; //
        pub const m_flFogVerticalExponent: usize = 0x588; //
        pub const m_fogColor: usize = 0x58C; //
        pub const m_flFogStrength: usize = 0x590; //
        pub const m_flFadeTime: usize = 0x594; //
        pub const m_bStartDisabled: usize = 0x598; //
        pub const m_bIsEnabled: usize = 0x599; //
        pub const m_bGradientFogNeedsTextures: usize = 0x59A; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Mirage_FireBeetles_Buff_VData {
        pub const m_CasterBuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_GenericPerson_4 {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityWreckerScrapBlastVData {
        pub const m_SprayParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChannelStartParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffModifier: usize = 0x1750; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_Item_Bleeding_Bullets_ActiveVData {
        pub const m_BleedModifier: usize = 0x668; //
        pub const m_BuildUpModifier: usize = 0x678; //
        pub const m_BulletImpactParticle: usize = 0x688; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Stimpak_regen {
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_BlastPush {
        pub const m_vPush: usize = 0xC0; //
        pub const m_flPushVelocity: usize = 0xCC; //
        pub const m_flMaxPushVelocity: usize = 0xD0; //
        pub const m_flMaxPushVelocitySqr: usize = 0xD4; //
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityProperty_WeaponDamage {
    }
    // Parent: CEntityComponent
    // Field count: 0
    pub mod CCitadelPlayerClipComponent {
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CItemAOESilenceAuraVData {
        pub const m_empWaveParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Disruptive_Charge {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_TargetPracticeDebuffVData {
        pub const m_SlowModifier: usize = 0x638; //
        pub const m_BulletResistModifier: usize = 0x648; //
        pub const m_EMPModifier: usize = 0x658; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 10
    pub mod CCitadel_Ability_FissureWall {
        pub const m_vecWallPreviewParticles: usize = 0xCB0; //
        pub const m_vecStartPos: usize = 0xD70; //
        pub const m_vecPosition: usize = 0xD7C; //
        pub const m_vecInitialPosition: usize = 0xD88; //
        pub const m_CastTime: usize = 0xD94; //
        pub const m_vecDirection: usize = 0xD98; //
        pub const m_vecLeft: usize = 0xDA4; //
        pub const m_Length: usize = 0xDB0; //
        pub const m_bTraveling: usize = 0xDB4; //
        pub const m_bPreview: usize = 0xDB5; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_LifestrikeGauntlets {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CModifier_CheatDeathImmunityVData {
        pub const m_BuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffPlayerParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StatusEffect: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIMaterial2>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_StatStealBaseVData {
        pub const m_StatStolenDebuffModifier: usize = 0x638; //
        pub const m_StatStolenBuffModifier: usize = 0x648; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod CCitadel_Projectile_Pillar {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Viper_PetrifyBola {
    }
    // Parent: CCitadelBaseShivAbility
    // Field count: 1
    pub mod CCitadel_Ability_Shiv_Defer_Damage {
        pub const m_flTotalPendingDamage: usize = 0xD80; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 10
    pub mod CCitadel_Ability_IceBeamVData {
        pub const m_BeamParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SlowModifier: usize = 0x1750; //
        pub const m_BuildupModifier: usize = 0x1760; //
        pub const m_BuildupProcModifier: usize = 0x1770; //
        pub const m_BeamStartSound: usize = 0x1780; //
        pub const m_BeamStopSound: usize = 0x1790; //
        pub const m_BeamPointStartLoopSound: usize = 0x17A0; //
        pub const m_BeamPointEndLoopSound: usize = 0x17B0; //
        pub const m_BeamPointClosestLoopSound: usize = 0x17C0; //
    }
    // Parent: C_NPC_TrooperBoss
    // Field count: 0
    pub mod C_NPC_TrooperBarrackBoss {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_MageWalk {
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Gunslinger_DemonMarkVData {
        pub const m_MarkModifier: usize = 0x1590; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 6
    pub mod CCitadel_Ability_ViscousWeapon_Alt {
        pub const m_ChargeState: usize = 0xCA0; //
        pub const m_nClipConsumed: usize = 0xCA4; //
        pub const m_bIsCharging: usize = 0xCA8; //
        pub const m_bIsToggled: usize = 0xCA9; //
        pub const m_fxChargingParticle: usize = 0xCAC; //
        pub const m_flLastBulletConsumedTime: usize = 0xCB8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_ColdFrontAOE {
        pub const m_hAOEEffect: usize = 0x1D8; //
    }
    // Parent: C_BaseCombatCharacter
    // Field count: 21
    pub mod C_PortraitWorldUnit {
        pub const m_bSuppressIntroEffects: usize = 0xD18; //
        pub const m_bIsAlternateLoadout: usize = 0xD19; //
        pub const m_bSpawnBackgroundModels: usize = 0xD1A; //
        pub const m_bDeferredPortrait: usize = 0xD1B; //
        pub const m_bShowParticleAssetModifiers: usize = 0xD1C; //
        pub const m_bIgnorePortraitInfo: usize = 0xD1D; //
        pub const m_bFlyingCourier: usize = 0xD1E; //
        pub const m_nEffigyStatusEffect: usize = 0xD20; //
        pub const m_effigySequenceName: usize = 0xD28; //
        pub const m_BodyGroup: usize = 0xD30; //
        pub const m_flStartingAnimationCycle: usize = 0xD38; //
        pub const m_flRareLoadoutAnimChance: usize = 0xD3C; //
        pub const m_environment: usize = 0xD50; // CitadelPortraitEnvironmentType_t
        pub const m_nStartupBehavior: usize = 0xD54; //
        pub const m_cameraName: usize = 0xEC8; //
        pub const m_nPortraitParticle: usize = 0xF00; //
        pub const m_nAmbientParticle: usize = 0xF04; //
        pub const m_nCourierType: usize = 0xF08; //
        pub const m_heroID: usize = 0xF0C; //
        pub const m_heroAnimGraphEnumName: usize = 0xF10; //
        pub const m_heroShopAnimGraphEnumName: usize = 0xF18; //
    }
    // Parent: C_EconEntity
    // Field count: 0
    pub mod C_EconWearable {
    }
    // Parent: C_BaseModelEntity
    // Field count: 4
    pub mod CCitadelBulletTimeWarp {
        pub const m_flBulletTimeScale: usize = 0x858; //
        pub const m_flProjectileTimeScale: usize = 0x85C; //
        pub const m_flExpireTime: usize = 0x860; //
        pub const m_flStopDuration: usize = 0x864; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifierChargedTackleActiveVData {
        pub const m_TackleParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PullEnemiesParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_AfterburnWatcher {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TriggerTowerRegen {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Item_HealthNova {
        pub const m_flAmountPerSecond: usize = 0xC0; //
    }
    // Parent: CCitadel_Modifier_Intrinsic_BaseVData
    // Field count: 1
    pub mod CCitadel_Modifier_MagicClarityWatcherVData {
        pub const m_BuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_TossUp {
        pub const m_bForceApplied: usize = 0xC0; //
        pub const m_bLandedOnGround: usize = 0xC1; //
        pub const m_vTossUpForce: usize = 0xC4; //
        pub const m_flCurrentVelocityScale: usize = 0xD0; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_AbilityLifeSteal {
    }
    // Parent: CCitadel_Item_TrackingProjectileApplyModifier
    // Field count: 0
    pub mod CCitadel_Item_RejuvTrackingProjectile {
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_ItemPickupPunchableVData {
        pub const m_flPhysicsRadius: usize = 0x638; //
        pub const m_IsDroppingParticle: usize = 0x640; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_IsPunchableParticle: usize = 0x720; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_IsFrozenParticle: usize = 0x800; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_NearRejuvAuraModifier: usize = 0x8E0; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_BaseToggle {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Astro_Rifle_Debuff {
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 4
    pub mod CCitadel_Modifier_LashGrappleEnemy_Debuff {
        pub const m_vCrashDir: usize = 0xC8; //
        pub const m_vLiftTarget: usize = 0xD4; //
        pub const m_flStartTime: usize = 0xE0; //
        pub const m_bCrashingDown: usize = 0xE4; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Healbane_Debuff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 9
    pub mod CCitadel_Modifier_RespawnCreditVData {
        pub const m_eRespawnMechanic: usize = 0x638; //
        pub const m_flRespawnDelay: usize = 0x63C; //
        pub const m_flBonusClipSize: usize = 0x640; //
        pub const m_flBonusFirerate: usize = 0x644; //
        pub const m_flBonusHealth: usize = 0x648; //
        pub const m_flBonusMoveSpeedMeterPerSecond: usize = 0x64C; //
        pub const m_sExpireSound: usize = 0x650; //
        pub const m_iMaxMessages: usize = 0x660; //
        pub const m_flMessageInterval: usize = 0x664; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CItem_FleetfootBoots {
    }
    // Parent: CCitadel_Modifier_Silenced
    // Field count: 2
    pub mod CCitadel_Modifier_Bubble {
        pub const m_flDampingFactor: usize = 0xC0; //
        pub const m_ParticleIndex: usize = 0x138; //
    }
    // Parent: C_BaseEntity
    // Field count: 18
    pub mod C_EnvVolumetricFogVolume {
        pub const m_bActive: usize = 0x560; //
        pub const m_vBoxMins: usize = 0x564; //
        pub const m_vBoxMaxs: usize = 0x570; //
        pub const m_bStartDisabled: usize = 0x57C; //
        pub const m_flStrength: usize = 0x580; //
        pub const m_nFalloffShape: usize = 0x584; //
        pub const m_flFalloffExponent: usize = 0x588; //
        pub const m_flHeightFogDepth: usize = 0x58C; //
        pub const m_fHeightFogEdgeWidth: usize = 0x590; //
        pub const m_fIndirectLightStrength: usize = 0x594; //
        pub const m_fSunLightStrength: usize = 0x598; //
        pub const m_fNoiseStrength: usize = 0x59C; //
        pub const m_TintColor: usize = 0x5A0; //
        pub const m_bOverrideTintColor: usize = 0x5A4; //
        pub const m_bOverrideIndirectLightStrength: usize = 0x5A5; //
        pub const m_bOverrideSunLightStrength: usize = 0x5A6; //
        pub const m_bOverrideNoiseStrength: usize = 0x5A7; //
        pub const m_bAllowLPVIndirect: usize = 0x5A8; //
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 2
    pub mod CModifierVandalSurgeVData {
        pub const m_LiftParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strStartSound: usize = 0x7F8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Viper_Ability04 {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_WreckerSalvage {
    }
    // Parent: CCitadelModifierVData
    // Field count: 9
    pub mod CCitadel_Modifier_TargetPracticeEnemyVData {
        pub const m_DebuffModifier: usize = 0x638; //
        pub const m_BuildupCompleteModifier: usize = 0x648; //
        pub const m_BuildupModifier: usize = 0x658; //
        pub const m_TargetParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x748; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HeadParticle: usize = 0x828; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strTargetHitSound: usize = 0x908; //
        pub const m_strTargetHeadShotHitSound: usize = 0x918; //
        pub const m_strTargetCompleteSound: usize = 0x928; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_Lash_Flog {
        pub const m_SandEffect: usize = 0xDB8; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_SiphonBulletsVData {
        pub const m_PermanentHealthLoss: usize = 0x15D8; //
    }
    // Parent: CCitadel_Modifier_ChainLightningEffectVData
    // Field count: 1
    pub mod CCitadel_Modifier_Galvanic_Storm_EffectVData {
        pub const m_BuffChainParticle: usize = 0x738; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_NearbyEnemyResistVData {
        pub const m_flNearbyEnemyResistRange: usize = 0x638; //
        pub const m_flResistValues: usize = 0x640; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_ModelPointEntity {
    }
    // Parent: C_BaseEntity
    // Field count: 7
    pub mod CPointOrient {
        pub const m_iszSpawnTargetName: usize = 0x560; //
        pub const m_hTarget: usize = 0x568; //
        pub const m_bActive: usize = 0x56C; //
        pub const m_nGoalDirection: usize = 0x570; //
        pub const m_nConstraint: usize = 0x574; //
        pub const m_flMaxTurnRate: usize = 0x578; //
        pub const m_flLastGameTime: usize = 0x57C; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_WarpStone_Caster {
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_Succor_MoveVData {
        pub const m_PullSound: usize = 0x638; //
        pub const m_flPullSpeedMin: usize = 0x648; //
        pub const m_flPullSpeedMax: usize = 0x64C; //
        pub const m_flPullDistanceMin: usize = 0x650; //
        pub const m_flPullDistanceMax: usize = 0x654; //
    }
    // Parent: CCitadelPlayerPawnBase
    // Field count: 44
    pub mod C_CitadelPlayerPawn {
        pub const m_angEyeAngles: usize = 0xF48; //
        pub const m_angClientCamera: usize = 0xF60; //
        pub const m_eZipLineLaneColor: usize = 0xF6C; //
        pub const m_nLevel: usize = 0xF70; //
        pub const m_nCurrencies: usize = 0xF74; //
        pub const m_nSpentCurrencies: usize = 0xF84; //
        pub const m_flLastSpawnTime: usize = 0xF94; //
        pub const m_flRespawnTime: usize = 0xF98; //
        pub const m_bInRegenerationZone: usize = 0xF9C; //
        pub const m_bInItemShopZone: usize = 0xF9D; //
        pub const m_timeRevealedOnMinimapByNPC: usize = 0xFA0; //
        pub const m_vecFullSellPriceItems: usize = 0xFA8; // C_NetworkUtlVectorBase<CUtlStringToken>
        pub const m_vecFullSellPriceAbilityUpgrades: usize = 0xFC0; //
        pub const m_bNetworkDisconnected: usize = 0xFD8; //
        pub const m_bHasIncomingThreats: usize = 0xFD9; //
        pub const m_bLearningAbility: usize = 0xFDA; //
        pub const m_nFlashStartTick: usize = 0xFDC; //
        pub const m_nFlashMaxStartTick: usize = 0xFE0; //
        pub const m_nFlashFadeStartTick: usize = 0xFE4; //
        pub const m_nFlashEndTick: usize = 0xFE8; //
        pub const m_nFlashMaxAlpha: usize = 0xFEC; //
        pub const m_nDeducedLane: usize = 0xFF0; //
        pub const m_nSuccessiveDucks: usize = 0xFF4; //
        pub const m_flLastDuckTime: usize = 0xFF8; //
        pub const m_bDismissedReportCard: usize = 0xFFC; //
        pub const m_flCurrentHealingAmount: usize = 0x1000; //
        pub const m_angLockedEyeAngles: usize = 0x1004; //
        pub const m_CCitadelAbilityComponent: usize = 0x1010; //
        pub const m_CCitadelHeroComponent: usize = 0x11B0; //
        pub const m_flRichPresenceUpdateInterval: usize = 0x1278; //
        pub const m_bAnimGraphMovementClipped: usize = 0x1370; //
        pub const m_bAnimGraphMovementDisableGravity: usize = 0x1371; //
        pub const m_bAnimGraphMovementDirectAirControl: usize = 0x1372; //
        pub const m_flPredTimeSlowedStart: usize = 0x1374; //
        pub const m_flPredTimeSlowedEnd: usize = 0x1378; //
        pub const m_flPredSlowSpeed: usize = 0x137C; //
        pub const m_flTimeSlowedStart: usize = 0x1380; //
        pub const m_flTimeSlowedEnd: usize = 0x1390; //
        pub const m_flSlowSpeed: usize = 0x13A0; //
        pub const m_flSprintAnimSuppressEndTime: usize = 0x13B0; //
        pub const m_iCurSlowSlot: usize = 0x13B4; //
        pub const m_vShootTestOffsetStanding: usize = 0x13B8; //
        pub const m_vShootTestOffsetCrouching: usize = 0x13C4; //
        pub const m_leanStartTime: usize = 0x13D0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Trapper_StealSpiritDebuff {
    }
    // Parent: CCitadel_Ability_ZipLine
    // Field count: 0
    pub mod CCitadel_Ability_TrooperZipLine {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Upgrade_Headhunter_HeadshotBuff {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 4
    pub mod CCitadel_Modifier_DetentionAmmoVData {
        pub const m_BuildUpModifier: usize = 0x668; //
        pub const m_DebuffModifier: usize = 0x678; //
        pub const m_ImmunityModifier: usize = 0x688; //
        pub const m_TracerParticle: usize = 0x698; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityPropertySingleStat {
    }
    // Parent: C_BaseFire
    // Field count: 13
    pub mod C_FireSmoke {
        pub const m_nFlameModelIndex: usize = 0x570; //
        pub const m_nFlameFromAboveModelIndex: usize = 0x574; //
        pub const m_flScaleRegister: usize = 0x578; //
        pub const m_flScaleStart: usize = 0x57C; //
        pub const m_flScaleEnd: usize = 0x580; //
        pub const m_flScaleTimeStart: usize = 0x584; //
        pub const m_flScaleTimeEnd: usize = 0x588; //
        pub const m_flChildFlameSpread: usize = 0x58C; //
        pub const m_flClipPerc: usize = 0x5A0; //
        pub const m_bClipTested: usize = 0x5A4; //
        pub const m_bFadingOut: usize = 0x5A5; //
        pub const m_tParticleSpawn: usize = 0x5A8; //
        pub const m_pFireOverlay: usize = 0x5B0; //
    }
    // Parent: C_Sprite
    // Field count: 0
    pub mod C_FireFromAboveSprite {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_UtilityUpgrade_AOESmokeBomb {
    }
    // Parent: C_BaseEntity
    // Field count: 24
    pub mod C_EnvCombinedLightProbeVolume {
        pub const m_Entity_Color: usize = 0x15C0; //
        pub const m_Entity_flBrightness: usize = 0x15C4; //
        pub const m_Entity_hCubemapTexture: usize = 0x15C8; //
        pub const m_Entity_bCustomCubemapTexture: usize = 0x15D0; //
        pub const m_Entity_hLightProbeTexture: usize = 0x15D8; //
        pub const m_Entity_hLightProbeDirectLightIndicesTexture: usize = 0x15E0; //
        pub const m_Entity_hLightProbeDirectLightScalarsTexture: usize = 0x15E8; //
        pub const m_Entity_hLightProbeDirectLightShadowsTexture: usize = 0x15F0; //
        pub const m_Entity_vBoxMins: usize = 0x15F8; //
        pub const m_Entity_vBoxMaxs: usize = 0x1604; //
        pub const m_Entity_bMoveable: usize = 0x1610; //
        pub const m_Entity_nHandshake: usize = 0x1614; //
        pub const m_Entity_nEnvCubeMapArrayIndex: usize = 0x1618; //
        pub const m_Entity_nPriority: usize = 0x161C; //
        pub const m_Entity_bStartDisabled: usize = 0x1620; //
        pub const m_Entity_flEdgeFadeDist: usize = 0x1624; //
        pub const m_Entity_vEdgeFadeDists: usize = 0x1628; //
        pub const m_Entity_nLightProbeSizeX: usize = 0x1634; //
        pub const m_Entity_nLightProbeSizeY: usize = 0x1638; //
        pub const m_Entity_nLightProbeSizeZ: usize = 0x163C; //
        pub const m_Entity_nLightProbeAtlasX: usize = 0x1640; //
        pub const m_Entity_nLightProbeAtlasY: usize = 0x1644; //
        pub const m_Entity_nLightProbeAtlasZ: usize = 0x1648; //
        pub const m_Entity_bEnabled: usize = 0x1661; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_Viper_VenomVData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_SleepDagger {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_FlameDash {
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityProperty_KineticCarbine {
    }
    // Parent: C_SoundOpvarSetPointBase
    // Field count: 0
    pub mod C_SoundOpvarSetOBBWindEntity {
    }
    // Parent: None
    // Field count: 4
    pub mod ActiveModelConfig_t {
        pub const m_Handle: usize = 0x30; //
        pub const m_Name: usize = 0x38; //
        pub const m_AssociatedEntities: usize = 0x40; //
        pub const m_AssociatedEntityNames: usize = 0x58; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_Mirage_Tornado_Lift_VData {
        pub const m_HoldInPlaceModifier: usize = 0x638; //
        pub const m_LiftParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_Fealty {
        pub const m_hTarget: usize = 0xCA0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_SummonGangster {
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_Gunslinger_KnockbackBlastVData {
        pub const m_ImpactParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WallImpactParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strWallSlamSound: usize = 0x1750; //
        pub const m_DebuffModifier: usize = 0x1760; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Bull_Leap_Boosting {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ZipLine_Boost {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_CheaterCurse {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Thumper_2_Aura {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierIntimidatedVData {
        pub const m_EffectParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Nano_ClusterGrenade {
        pub const m_vecHitEnemies: usize = 0xCA0; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_flNextProjectileTime: usize = 0xCB8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Nano_CatForm {
        pub const m_bIsInCatform: usize = 0xCA0; //
        pub const m_flLastDamageTime: usize = 0xCA4; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_Upgrade_AerialAssualtVData {
        pub const m_WatcherModifier: usize = 0x15D8; //
        pub const m_LaunchParticle: usize = 0x15E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Unstoppable {
        pub const m_bInCheckState: usize = 0xC0; //
    }
    // Parent: C_DynamicProp
    // Field count: 0
    pub mod C_DynamicPropAlias_dynamic_prop {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierVData_SetMoveType {
        pub const m_nMoveType: usize = 0x638; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_StickyBomb {
        pub const m_flBombBonus: usize = 0xCA8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_LightningBullet {
    }
    // Parent: CCitadel_UtilityUpgrade_RocketBootsVData
    // Field count: 7
    pub mod CCitadel_UtilityUpgrade_RocketBoosterVData {
        pub const m_LandingParticle: usize = 0x16C8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AoEPreviewParticle: usize = 0x17A8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DropDownStartParticle: usize = 0x1888; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DropDownStartSound: usize = 0x1968; //
        pub const m_LandingSound: usize = 0x1978; //
        pub const m_DebuffModifier: usize = 0x1988; //
        pub const m_flSlamEnabledTime: usize = 0x1998; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Magic_Clarity_BuffVData {
        pub const m_VisualModifier: usize = 0x638; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_AcolytesGlove_VData {
        pub const m_DebuffModifier: usize = 0x668; //
        pub const m_SwingParticle: usize = 0x678; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x758; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: None
    // Field count: 43
    pub mod PlayerDataGlobal_t {
        pub const m_iLevel: usize = 0x8; //
        pub const m_iMaxAmmo: usize = 0xC; //
        pub const m_iHealthMax: usize = 0x10; //
        pub const m_flHealthRegen: usize = 0x14; //
        pub const m_flRespawnTime: usize = 0x18; //
        pub const m_nHeroID: usize = 0x1C; //
        pub const m_iGoldNetWorth: usize = 0x20; //
        pub const m_iAPNetWorth: usize = 0x24; //
        pub const m_iCreepGold: usize = 0x28; //
        pub const m_iCreepGoldSoloBonus: usize = 0x2C; //
        pub const m_iCreepGoldKill: usize = 0x30; //
        pub const m_iCreepGoldAirOrb: usize = 0x34; //
        pub const m_iCreepGoldGroundOrb: usize = 0x38; //
        pub const m_iCreepGoldDeny: usize = 0x3C; //
        pub const m_iCreepGoldNeutral: usize = 0x40; //
        pub const m_iFarmBaseline: usize = 0x44; //
        pub const m_iHealth: usize = 0x48; //
        pub const m_iPlayerKills: usize = 0x4C; //
        pub const m_iPlayerAssists: usize = 0x50; //
        pub const m_iDeaths: usize = 0x54; //
        pub const m_iDenies: usize = 0x58; //
        pub const m_iLastHits: usize = 0x5C; //
        pub const m_bAlive: usize = 0x60; //
        pub const m_nHeroDraftPosition: usize = 0x64; //
        pub const m_bUltimateTrained: usize = 0x68; //
        pub const m_flUltimateCooldownStart: usize = 0x6C; //
        pub const m_flUltimateCooldownEnd: usize = 0x70; //
        pub const m_bHasRejuvenator: usize = 0x74; //
        pub const m_bHasRebirth: usize = 0x75; //
        pub const m_bFlaggedAsCheater: usize = 0x76; //
        pub const m_iHeroDamage: usize = 0x78; //
        pub const m_iHeroHealing: usize = 0x7C; //
        pub const m_iSelfHealing: usize = 0x80; //
        pub const m_iObjectiveDamage: usize = 0x84; //
        pub const m_nHeroAbilityUpgradeBits: usize = 0x88; //
        pub const m_vecUpgrades: usize = 0x98; //
        pub const m_vecBonusCounterAbilities: usize = 0xB0; //
        pub const m_vecBonusCounterValues: usize = 0xC8; //
        pub const m_tHeldItem: usize = 0xE0; //
        pub const m_vecImbuements: usize = 0xE8; //
        pub const m_vecDynamicAbilityValues: usize = 0x150; //
        pub const m_vecStatViewerModifierValues: usize = 0x1B8; //
        pub const m_vecStolenAbilities: usize = 0x220; //
    }
    // Parent: CLogicalEntity
    // Field count: 5
    pub mod CLogicRelay {
        pub const m_bDisabled: usize = 0x560; //
        pub const m_bWaitForRefire: usize = 0x561; //
        pub const m_bTriggerOnce: usize = 0x562; //
        pub const m_bFastRetrigger: usize = 0x563; //
        pub const m_bPassthoughCaller: usize = 0x564; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CAbility_Mirage_Tornado_VData {
        pub const m_TornadoCastParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CasterModifier: usize = 0x1670; //
        pub const m_WhirlwindEvasionModifier: usize = 0x1680; //
        pub const m_TornadoAura: usize = 0x1690; //
        pub const m_GrenadeTrailModifier: usize = 0x16A0; //
        pub const m_cameraSequenceTravelingInTornado: usize = 0x16B0; //
    }
    // Parent: CCitadelBaseYamatoAbility
    // Field count: 2
    pub mod CCitadel_Ability_PowerSlash {
        pub const m_nPowerLevel: usize = 0xCB4; //
        pub const m_nCastParticle: usize = 0xCB8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Bomber_Ability02 {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_HealthSwap {
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 0
    pub mod CTier3BossAbility {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Skirmish_Respawn_Reduction {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Base {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_BurstFire {
        pub const m_nFastFireEndTime: usize = 0xCB8; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_HealOnLevel {
    }
    // Parent: C_PointCamera
    // Field count: 1
    pub mod C_PointCameraVFOV {
        pub const m_flVerticalFOV: usize = 0x5C0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_AnimalCurse {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Synth_Barrage_Caster {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CAbility_Synth_Pulse {
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 3
    pub mod CCitadel_Modifier_VacuumAuraTargetModifierVData {
        pub const m_flOuterSpeedScale: usize = 0x718; //
        pub const m_flSpeedScaleBias: usize = 0x71C; //
        pub const m_TargetLoopingSound: usize = 0x720; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Chrono_KineticCarbine_Slow {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PowerJump {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CModifier_Upgrade_ArcaneMedallion {
    }
    // Parent: C_BaseTrigger
    // Field count: 1
    pub mod C_CitadelTeleportTrigger {
        pub const m_vExitOrigin: usize = 0x998; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 74
    pub mod C_BarnLight {
        pub const m_bEnabled: usize = 0x858; //
        pub const m_nColorMode: usize = 0x85C; //
        pub const m_Color: usize = 0x860; //
        pub const m_flColorTemperature: usize = 0x864; //
        pub const m_flBrightness: usize = 0x868; //
        pub const m_flBrightnessScale: usize = 0x86C; //
        pub const m_nDirectLight: usize = 0x870; //
        pub const m_nBakedShadowIndex: usize = 0x874; //
        pub const m_nLuminaireShape: usize = 0x878; //
        pub const m_flLuminaireSize: usize = 0x87C; //
        pub const m_flLuminaireAnisotropy: usize = 0x880; //
        pub const m_LightStyleString: usize = 0x888; //
        pub const m_flLightStyleStartTime: usize = 0x890; //
        pub const m_QueuedLightStyleStrings: usize = 0x898; // C_NetworkUtlVectorBase<CUtlString>
        pub const m_LightStyleEvents: usize = 0x8B0; // C_NetworkUtlVectorBase<CUtlString>
        pub const m_LightStyleTargets: usize = 0x8C8; //
        pub const m_StyleEvent: usize = 0x8E0; //
        pub const m_hLightCookie: usize = 0x980; //
        pub const m_flShape: usize = 0x988; //
        pub const m_flSoftX: usize = 0x98C; //
        pub const m_flSoftY: usize = 0x990; //
        pub const m_flSkirt: usize = 0x994; //
        pub const m_flSkirtNear: usize = 0x998; //
        pub const m_vSizeParams: usize = 0x99C; //
        pub const m_flRange: usize = 0x9A8; //
        pub const m_vShear: usize = 0x9AC; //
        pub const m_nBakeSpecularToCubemaps: usize = 0x9B8; //
        pub const m_vBakeSpecularToCubemapsSize: usize = 0x9BC; //
        pub const m_nCastShadows: usize = 0x9C8; //
        pub const m_nShadowMapSize: usize = 0x9CC; //
        pub const m_nShadowPriority: usize = 0x9D0; //
        pub const m_bContactShadow: usize = 0x9D4; //
        pub const m_bForceShadowsEnabled: usize = 0x9D5; //
        pub const m_nBounceLight: usize = 0x9D8; //
        pub const m_flBounceScale: usize = 0x9DC; //
        pub const m_flMinRoughness: usize = 0x9E0; //
        pub const m_vAlternateColor: usize = 0x9E4; //
        pub const m_fAlternateColorBrightness: usize = 0x9F0; //
        pub const m_nFog: usize = 0x9F4; //
        pub const m_flFogStrength: usize = 0x9F8; //
        pub const m_nFogShadows: usize = 0x9FC; //
        pub const m_flFogScale: usize = 0xA00; //
        pub const m_bFogMixedShadows: usize = 0xA04; //
        pub const m_flFadeSizeStart: usize = 0xA08; //
        pub const m_flFadeSizeEnd: usize = 0xA0C; //
        pub const m_flShadowFadeSizeStart: usize = 0xA10; //
        pub const m_flShadowFadeSizeEnd: usize = 0xA14; //
        pub const m_bPrecomputedFieldsValid: usize = 0xA18; //
        pub const m_vPrecomputedBoundsMins: usize = 0xA1C; //
        pub const m_vPrecomputedBoundsMaxs: usize = 0xA28; //
        pub const m_vPrecomputedOBBOrigin: usize = 0xA34; //
        pub const m_vPrecomputedOBBAngles: usize = 0xA40; //
        pub const m_vPrecomputedOBBExtent: usize = 0xA4C; //
        pub const m_nPrecomputedSubFrusta: usize = 0xA58; //
        pub const m_vPrecomputedOBBOrigin0: usize = 0xA5C; //
        pub const m_vPrecomputedOBBAngles0: usize = 0xA68; //
        pub const m_vPrecomputedOBBExtent0: usize = 0xA74; //
        pub const m_vPrecomputedOBBOrigin1: usize = 0xA80; //
        pub const m_vPrecomputedOBBAngles1: usize = 0xA8C; //
        pub const m_vPrecomputedOBBExtent1: usize = 0xA98; //
        pub const m_vPrecomputedOBBOrigin2: usize = 0xAA4; //
        pub const m_vPrecomputedOBBAngles2: usize = 0xAB0; //
        pub const m_vPrecomputedOBBExtent2: usize = 0xABC; //
        pub const m_vPrecomputedOBBOrigin3: usize = 0xAC8; //
        pub const m_vPrecomputedOBBAngles3: usize = 0xAD4; //
        pub const m_vPrecomputedOBBExtent3: usize = 0xAE0; //
        pub const m_vPrecomputedOBBOrigin4: usize = 0xAEC; //
        pub const m_vPrecomputedOBBAngles4: usize = 0xAF8; //
        pub const m_vPrecomputedOBBExtent4: usize = 0xB04; //
        pub const m_vPrecomputedOBBOrigin5: usize = 0xB10; //
        pub const m_vPrecomputedOBBAngles5: usize = 0xB1C; //
        pub const m_vPrecomputedOBBExtent5: usize = 0xB28; //
        pub const m_bInitialBoneSetup: usize = 0xB78; //
        pub const m_VisClusters: usize = 0xB80; // C_NetworkUtlVectorBase<uint16>
    }
    // Parent: C_BaseEntity
    // Field count: 8
    pub mod C_TonemapController2 {
        pub const m_flAutoExposureMin: usize = 0x560; //
        pub const m_flAutoExposureMax: usize = 0x564; //
        pub const m_flTonemapPercentTarget: usize = 0x568; //
        pub const m_flTonemapPercentBrightPixels: usize = 0x56C; //
        pub const m_flTonemapMinAvgLum: usize = 0x570; //
        pub const m_flExposureAdaptationSpeedUp: usize = 0x574; //
        pub const m_flExposureAdaptationSpeedDown: usize = 0x578; //
        pub const m_flTonemapEVSmoothingRange: usize = 0x57C; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_SpinVData {
        pub const m_AoEParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SlowModifier: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbilityIntimidateVData {
        pub const m_EnemyModifier: usize = 0x1590; //
        pub const m_DebuffModifier: usize = 0x15A0; //
        pub const m_AoEPlayerParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AoEParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CModifier_Upgrade_ArcaneSurge_AbilityWatcher_VData {
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 2
    pub mod CCitadel_MobileResupply {
        pub const m_hAbility: usize = 0xB00; //
        pub const m_bFloating: usize = 0xB04; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_HauntWatcher {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Wraith_RapidFireVData {
        pub const m_RapidFireParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_HornetLeap {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 5
    pub mod CCitadel_Modifier_SilenceProcWatcherVData {
        pub const m_BuildUpModifier: usize = 0x668; //
        pub const m_DebuffModifier: usize = 0x678; //
        pub const m_ImmunityModifier: usize = 0x688; //
        pub const m_sInstantProcIfCasterHasModifier: usize = 0x698; //
        pub const m_TracerParticle: usize = 0x6A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 82
    pub mod CitadelAbilityVData {
        pub const m_eAbilityType: usize = 0x28; //
        pub const m_eItemSlotType: usize = 0x29; //
        pub const m_bDisabled: usize = 0x2A; //
        pub const m_bInDevelopment: usize = 0x2B; //
        pub const m_bStartTrained: usize = 0x2C; //
        pub const m_iMaxLevel: usize = 0x30; //
        pub const m_nAbilityPointsCost: usize = 0x34; //
        pub const m_nAbillityUnlocksCost: usize = 0x38; //
        pub const m_iUpdateTime: usize = 0x40; //
        pub const m_AbilityBehaviorsBits: usize = 0x4C; // CBitVecEnum<EAbilityBehavior_t>
        pub const m_eAbilityTargetingLocation: usize = 0x58; //
        pub const m_eAbilityTargetingShape: usize = 0x5C; //
        pub const m_flTargetingConeAngle: usize = 0x60; //
        pub const m_flTargetingConeHalfWidth: usize = 0x64; //
        pub const m_bIncludeExtra2DCone: usize = 0x68; //
        pub const m_eAbilityActivation: usize = 0x6C; //
        pub const m_TriggerButtonPreReqButton: usize = 0x70; //
        pub const m_TriggerButtonOverride: usize = 0x78; //
        pub const m_eAbilitySpectatePriority: usize = 0x80; //
        pub const m_bitsInterruptingStates: usize = 0x84; //
        pub const m_IncompatibleFilter: usize = 0xA0; //
        pub const m_nAbilityTargetTypes: usize = 0xB4; //
        pub const m_nAbilityTargetFlags: usize = 0xB8; //
        pub const m_bitsPostCastEnabledStateMask: usize = 0xBC; //
        pub const m_TargetAbilityEffectsToApply: usize = 0xD8; //
        pub const m_flBossDamageScale: usize = 0xDC; //
        pub const m_bShowTargetingPreviewWhileChanneling: usize = 0xE0; //
        pub const m_bShowTargetingPreviewWhileCasting: usize = 0xE1; //
        pub const m_WeaponInfo: usize = 0xE8; //
        pub const m_projectileInfo: usize = 0x768; //
        pub const m_deploymentInfo: usize = 0xB08; //
        pub const m_mapAbilityProperties: usize = 0xCE8; //
        pub const m_vecDependentAbilities: usize = 0xD10; //
        pub const m_vecAbilityUpgrades: usize = 0xD28; //
        pub const m_strCastAnimGraphParam: usize = 0xD60; //
        pub const m_strSelectionNameOverride: usize = 0xD68; //
        pub const m_strCastAnimSequenceName: usize = 0xD70; //
        pub const m_AbilityTooltipDetails: usize = 0xD78; //
        pub const m_strCSSClass: usize = 0xDA8; //
        pub const m_strAbilityImage: usize = 0xDB0; //
        pub const m_strMoviePreviewPath: usize = 0xDC0; //
        pub const m_HUDPanel: usize = 0xDC8; //
        pub const m_bShowInPassiveItemsArea: usize = 0xE00; //
        pub const m_bForceShowHUDPanel: usize = 0xE01; //
        pub const m_bUsesFlightControls: usize = 0xE02; //
        pub const m_strFlyUpLocString: usize = 0xE08; //
        pub const m_strFlyDownLocString: usize = 0xE10; //
        pub const m_additionalAbilities: usize = 0xE18; //
        pub const m_strCancelAbilityKey: usize = 0xE38; //
        pub const m_strSecondaryStatName: usize = 0xE40; //
        pub const m_strCastButtonLocToken: usize = 0xE48; //
        pub const m_strAltCastButtonLocToken: usize = 0xE50; //
        pub const m_cameraSequenceCastStart: usize = 0xE58; //
        pub const m_bEndCastStartSequenceOnCastComplete: usize = 0xEE0; //
        pub const m_cameraSequenceCastComplete: usize = 0xEE8; //
        pub const m_cameraSequenceChannelStart: usize = 0xF70; //
        pub const m_bEndChannelStartSequenceOnChannelComplete: usize = 0xFF8; //
        pub const m_flCameraPreviewOffset: usize = 0xFFC; //
        pub const m_flCameraPreviewDistance: usize = 0x1000; //
        pub const m_flCameraPreviewSpeed: usize = 0x1004; //
        pub const m_previewParticle: usize = 0x1008; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PreviewPathParticle: usize = 0x10E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_mapCastEventParticles: usize = 0x11C8; //
        pub const m_skillshotHitParticle: usize = 0x11F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_skillshotMissParticle: usize = 0x12D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetingPreviewParticle: usize = 0x13B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strSelectedSound: usize = 0x1490; //
        pub const m_strUnselectedSound: usize = 0x14A0; //
        pub const m_strSelectedLoopSound: usize = 0x14B0; //
        pub const m_strCastSound: usize = 0x14C0; //
        pub const m_strChannelSound: usize = 0x14D0; //
        pub const m_strChannelLoopSound: usize = 0x14E0; //
        pub const m_strCastDelaySound: usize = 0x14F0; //
        pub const m_strCastDelayLoopSound: usize = 0x1500; //
        pub const m_strHitConfirmationSound: usize = 0x1510; //
        pub const m_strDamageTakenSound: usize = 0x1520; //
        pub const m_strAbilityOffCooldownSound: usize = 0x1530; //
        pub const m_strAbilityChargeReadySound: usize = 0x1540; //
        pub const m_bPlayMeepMop: usize = 0x1550; //
        pub const m_AutoChannelModifier: usize = 0x1558; //
        pub const m_AutoCastDelayModifier: usize = 0x1568; //
        pub const m_AutoIntrinsicModifiers: usize = 0x1578; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TrooperDisabledInvulnerabilityFX {
    }
    // Parent: C_SoundOpvarSetPointEntity
    // Field count: 0
    pub mod C_SoundOpvarSetAutoRoomEntity {
    }
    // Parent: CCitadel_UtilityUpgrade_RocketBoots
    // Field count: 7
    pub mod CCitadel_UtilityUpgrade_RocketBooster {
        pub const m_nTargetingParticleIndex: usize = 0xD2C; //
        pub const m_flCastTime: usize = 0xD30; //
        pub const m_bCrashingDown: usize = 0xD34; //
        pub const m_bImpulseApplied: usize = 0xD35; //
        pub const m_bCanCrash: usize = 0xD36; //
        pub const m_vecCrashPosition: usize = 0xD38; //
        pub const m_vecCrashDirection: usize = 0xD44; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_SelfBuffModifier {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Cadence_Anthem {
    }
    // Parent: CCitadelYamatoBaseVData
    // Field count: 21
    pub mod CAbilityPowerSlashVData {
        pub const m_flAirDrag: usize = 0x1598; //
        pub const m_flMaxPowerPadding: usize = 0x159C; //
        pub const m_flEffectGroundTrace: usize = 0x15A0; //
        pub const m_flWhizbyMaxRange: usize = 0x15A4; //
        pub const m_flStartPosTestCapsuleLength: usize = 0x15A8; //
        pub const m_flCoverLOSBackDist: usize = 0x15AC; //
        pub const m_vecLongEffectOffset: usize = 0x15B0; //
        pub const m_PowerSlashParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PowerSlashFullParticle: usize = 0x16A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x1780; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1860; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PowerUpParticle: usize = 0x1940; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strStartSound: usize = 0x1A20; //
        pub const m_strHitConfirmSound: usize = 0x1A30; //
        pub const m_strPowerUp1Sounds: usize = 0x1A40; //
        pub const m_strPowerUp2Sounds: usize = 0x1A50; //
        pub const m_strPowerUp3Sounds: usize = 0x1A60; //
        pub const m_strWhizbySound: usize = 0x1A70; //
        pub const m_strSlashSound: usize = 0x1A80; //
        pub const m_strSlashFullSound: usize = 0x1A90; //
        pub const m_SlowModifier: usize = 0x1AA0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_IceGrenadeVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_IceGrenadeSlowModifier: usize = 0x1670; //
        pub const m_ExplosionSound: usize = 0x1680; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_ReturnFireVData {
        pub const m_ReactiveArmorModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_TechShieldImpact {
        pub const m_AmbientEffect: usize = 0xC0; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod C_NPC_PestilenceDrone {
    }
    // Parent: C_BaseFlex
    // Field count: 12
    pub mod C_EconEntity {
        pub const m_AttributeManager: usize = 0xCA0; //
        pub const m_bClientside: usize = 0xDF8; //
        pub const m_nDisableMode: usize = 0xDFC; //
        pub const m_bParticleSystemsCreated: usize = 0xE00; //
        pub const m_bForceDestroyAttachedParticlesImmediately: usize = 0xE01; //
        pub const m_vecAttachedParticles: usize = 0xE08; //
        pub const m_hViewmodelAttachment: usize = 0xE20; //
        pub const m_iOldTeam: usize = 0xE24; //
        pub const m_bAttachmentDirty: usize = 0xE28; //
        pub const m_iOldStyle: usize = 0xE29; //
        pub const m_hOldProvidee: usize = 0xE2C; //
        pub const m_vecAttachedModels: usize = 0xE30; // CUtlVector<C_EconEntity::AttachedModelData_t>
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 0
    pub mod CCitadel_Modifier_Tokamak_EnemySmokeAOE_VData {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Citadel_Projectile_DustStorm {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Trapper_PoisonJarVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AuraModifier: usize = 0x1670; //
        pub const m_ExplodeSound: usize = 0x1680; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_Viper_PetrifyBolaVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SlowModifier: usize = 0x1670; //
        pub const m_PetrifyModifier: usize = 0x1680; //
        pub const m_strBolaExplodeSound: usize = 0x1690; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_WeaponUpgrade_SurgingPower {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_SmokeBomb {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_ChargedShot {
        pub const m_ChannelParticle: usize = 0xCA0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Tier2Boss_RocketDamage_AuraDebuff {
    }
    // Parent: C_BarnLight
    // Field count: 1
    pub mod C_RectLight {
        pub const m_bShowLight: usize = 0xBA0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityCadenceAnthemVData {
        pub const m_AnthemAOEModifier: usize = 0x1590; //
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 2
    pub mod CModifierVandalOverflowVData {
        pub const m_LiftParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strStartSound: usize = 0x7F8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CModifier_FleetfootBoots_BonusClip {
        pub const m_nBonusClip: usize = 0xC0; //
    }
    // Parent: CCitadel_Modifier_StatStealBase
    // Field count: 0
    pub mod CCitadel_Modifier_Siphon_Bullets_Watcher {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Glitch {
    }
    // Parent: C_BaseEntity
    // Field count: 3
    pub mod CCitadelHeroLoader {
        pub const m_hero: usize = 0x560; //
        pub const m_nLoadSeq: usize = 0x568; //
        pub const m_hOwner: usize = 0x56C; //
    }
    // Parent: CCitadelModifierAura_ConeVData
    // Field count: 1
    pub mod CCitadel_Modifier_Fathom_ScaldingSpray_Aura_VData {
        pub const m_BuffModifier: usize = 0x688; //
    }
    // Parent: CCitadel_Modifier_Disarmed
    // Field count: 0
    pub mod CCitadel_Modifier_DisarmProc {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_VexBarrier {
    }
    // Parent: C_BaseModelEntity
    // Field count: 24
    pub mod C_ParticleSystem {
        pub const m_szSnapshotFileName: usize = 0x858; //
        pub const m_bActive: usize = 0xA58; //
        pub const m_bFrozen: usize = 0xA59; //
        pub const m_flFreezeTransitionDuration: usize = 0xA5C; //
        pub const m_nStopType: usize = 0xA60; //
        pub const m_bAnimateDuringGameplayPause: usize = 0xA64; //
        pub const m_iEffectIndex: usize = 0xA68; //
        pub const m_flStartTime: usize = 0xA70; //
        pub const m_flPreSimTime: usize = 0xA74; //
        pub const m_vServerControlPoints: usize = 0xA78; //
        pub const m_iServerControlPointAssignments: usize = 0xAA8; //
        pub const m_hControlPointEnts: usize = 0xAAC; //
        pub const m_bNoSave: usize = 0xBAC; //
        pub const m_bNoFreeze: usize = 0xBAD; //
        pub const m_bNoRamp: usize = 0xBAE; //
        pub const m_bStartActive: usize = 0xBAF; //
        pub const m_iszEffectName: usize = 0xBB0; //
        pub const m_iszControlPointNames: usize = 0xBB8; //
        pub const m_nDataCP: usize = 0xDB8; //
        pub const m_vecDataCPValue: usize = 0xDBC; //
        pub const m_nTintCP: usize = 0xDC8; //
        pub const m_clrTint: usize = 0xDCC; //
        pub const m_bOldActive: usize = 0xDF0; //
        pub const m_bOldFrozen: usize = 0xDF1; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 8
    pub mod CModifier_Wrecker_UltimateVData {
        pub const m_EnemyGrabModifier: usize = 0x638; //
        pub const m_EnemyThrowModifier: usize = 0x648; //
        pub const m_EnemyDamageModifier: usize = 0x658; //
        pub const m_InvincibleModifier: usize = 0x668; //
        pub const m_StartSound: usize = 0x678; //
        pub const m_AmbientLoopingSound: usize = 0x688; //
        pub const m_GrabSound: usize = 0x698; //
        pub const m_ThrowSound: usize = 0x6A8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Low_Health_Glow {
        pub const m_nFXIndex: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Magic_Clarity_Buff {
        pub const m_bAbilityLocked: usize = 0xF8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Thumper_3 {
    }
    // Parent: CCitadelModifierVData
    // Field count: 9
    pub mod CCitadel_Modifier_StickyBombAttachedVData {
        pub const m_BombAttachedParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StunAttachedParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BombAttachedVictimTeamParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExplodeSound: usize = 0x9B8; //
        pub const m_strTickTockSound: usize = 0x9C8; //
        pub const m_strTickTockFastSound: usize = 0x9D8; //
        pub const m_DebuffModifier: usize = 0x9E8; //
        pub const m_DetonateWarningTime: usize = 0x9F8; //
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 6
    pub mod CModifierLashGrappleEnemyDebuffVData {
        pub const m_GrappleParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LaunchParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RopeParticle: usize = 0x9B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactSound: usize = 0xA98; //
        pub const m_DebuffModifier: usize = 0xAA8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SpeedBoost {
    }
    // Parent: CEnvSoundscapeTriggerable
    // Field count: 0
    pub mod CEnvSoundscapeTriggerableAlias_snd_soundscape_triggerable {
    }
    // Parent: CBaseLockonAbilityVData
    // Field count: 15
    pub mod CAbilityLashUltimateVData {
        pub const m_TargetPreviewParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LaunchParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_UltimateCastParticle: usize = 0x1770; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_UltimateCastEnemyParticle: usize = 0x1850; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AllyIndicatorParticle: usize = 0x1930; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strThrowEnemyAnimGraphParam: usize = 0x1A10; //
        pub const m_GrappleEnemyModifier: usize = 0x1A18; //
        pub const m_GrabSound: usize = 0x1A28; //
        pub const m_MissSound: usize = 0x1A38; //
        pub const m_ThrowSound: usize = 0x1A48; //
        pub const m_flAirSpeedMax: usize = 0x1A58; //
        pub const m_flFallSpeedMax: usize = 0x1A5C; //
        pub const m_flAirDrag: usize = 0x1A60; //
        pub const m_flMaxPitchRangeScale: usize = 0x1A64; //
        pub const m_flThrowAnimTossPoint: usize = 0x1A68; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_Crackshot {
        pub const m_ReadyParticleIndex: usize = 0xCA4; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Bomber_Ability03 {
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityProperty_NanoTechRoundsDamage {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_Stimpak {
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 2
    pub mod CCitadel_Modifier_Knockback {
        pub const m_flForce: usize = 0xC8; //
        pub const m_bKnockedBack: usize = 0xCC; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 10
    pub mod CAbility_Fathom_ReefdwellerHarpoon {
        pub const m_bHitTarget: usize = 0xCA0; //
        pub const m_vPrevPos: usize = 0xCA4; //
        pub const m_bBulletFlying: usize = 0xCB0; //
        pub const m_bHasLatchedOnce: usize = 0xCB1; //
        pub const m_bLatched: usize = 0xCB2; //
        pub const m_vHarpoonTarget: usize = 0xCB4; //
        pub const m_flLatchedYaw: usize = 0xCC0; //
        pub const m_flCloseEnoughStartTime: usize = 0xCC4; //
        pub const m_flStuckStartTime: usize = 0xCC8; //
        pub const m_flReelStartTime: usize = 0xCCC; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CAbility_Synth_Grasp {
        pub const m_vecTetheredEnemies: usize = 0xCA0; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CAbilityGarbageVData {
        pub const m_GarbageAuraModifier: usize = 0x1590; //
        pub const m_ExplodeParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flAirSpeedMax: usize = 0x1680; //
        pub const m_flFallSpeedMax: usize = 0x1684; //
        pub const m_flAirDrag: usize = 0x1688; //
        pub const m_flMaxMovespeed: usize = 0x168C; //
    }
    // Parent: CPlayerPawnComponent
    // Field count: 6
    pub mod CPlayer_ObserverServices {
        pub const m_iObserverMode: usize = 0x40; //
        pub const m_hObserverTarget: usize = 0x44; //
        pub const m_iObserverLastMode: usize = 0x48; //
        pub const m_bForcedObserverMode: usize = 0x4C; //
        pub const m_flObserverChaseDistance: usize = 0x50; //
        pub const m_flObserverChaseDistanceCalcTime: usize = 0x54; //
    }
    // Parent: CCitadelBaseTriggerAbility
    // Field count: 1
    pub mod CCitadel_Ability_TangoTether_Trigger {
        pub const m_hBaseAbility: usize = 0xCB4; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 1
    pub mod C_AssignedLaneParticle {
        pub const m_iLane: usize = 0x878; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityThumper4VData {
        pub const m_PullAOEModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Bounce_Pad_Stomp {
        pub const m_bStomped: usize = 0x2F0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ChargedBomb {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_IncendiaryThinker {
    }
    // Parent: CPlayerPawnComponent
    // Field count: 0
    pub mod CPlayer_WaterServices {
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 2
    pub mod C_Citadel_Nano_Predatory_Statue {
        pub const m_hAbility: usize = 0xB08; //
        pub const m_flLifetime: usize = 0xB0C; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_FearWatcher {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadelBaseYamatoAbility {
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 2
    pub mod CCitadel_Modifier_VandalOverflow {
        pub const m_vecFloatDest: usize = 0x138; //
        pub const m_vecStartingPos: usize = 0x144; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SleepDagger_Drowsy {
    }
    // Parent: CCitadel_Modifier_ShieldTracker_Base
    // Field count: 0
    pub mod CCitadel_Modifier_ShieldTracker_Magic {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_CanDamageTier3Phase2 {
    }
    // Parent: CCitadelBaseDashCastAbility
    // Field count: 0
    pub mod CCitadel_Ability_Cadence_SilenceContraptions {
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CCitadel_Ability_ShadowPulse_VData {
        pub const m_EscapeModifier: usize = 0x1590; //
        pub const m_DebuffModifier: usize = 0x15A0; //
        pub const m_StartAoEParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExitAoEParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EffectParticle: usize = 0x1770; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x1850; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RadiusParticle: usize = 0x1930; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExpireSound: usize = 0x1A10; //
        pub const m_cameraSequenceInShadow: usize = 0x1A20; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 29
    pub mod CAbilityLashDownStrikeVData {
        pub const m_TargetPreviewParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strGroundCastAnimGraphParam: usize = 0x1670; //
        pub const m_strAirCastAnimGraphParam: usize = 0x1678; //
        pub const m_StompParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StompLineParticle: usize = 0x1760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StompLineObstructedParticle: usize = 0x1840; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StompImpactParticle: usize = 0x1920; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StompExplosionSound: usize = 0x1A00; //
        pub const m_StompEnemyImpactSound: usize = 0x1A10; //
        pub const m_strFallCollideImpactSound: usize = 0x1A20; //
        pub const m_DownStrikeModifier: usize = 0x1A30; //
        pub const m_ImpactModifier: usize = 0x1A40; //
        pub const m_DragModifier: usize = 0x1A50; //
        pub const m_flHeightUILingerTime: usize = 0x1A60; //
        pub const m_flDamageFrustumHalfWidth: usize = 0x1A64; //
        pub const m_flDamageFrustumAngle: usize = 0x1A68; //
        pub const m_flDamageWaveSpeed: usize = 0x1A6C; //
        pub const m_flDamageTraceProbeDamageRadius: usize = 0x1A70; //
        pub const m_flDamageTraceProbeWorldRadius: usize = 0x1A74; //
        pub const m_flDamageTraceProbeStepUpHeight: usize = 0x1A78; //
        pub const m_flDamageTraceProbeStepDownHeight: usize = 0x1A7C; //
        pub const m_flDamageTraceProbeDropDownRate: usize = 0x1A80; //
        pub const m_flInitialDamageRadiusInMeters: usize = 0x1A84; //
        pub const m_nGroundCrackGap: usize = 0x1A88; //
        pub const m_flGroupLengthTolerance: usize = 0x1A8C; //
        pub const m_flDamageEffectScaleMin: usize = 0x1A90; //
        pub const m_flDamageEffectScaleMax: usize = 0x1A94; //
        pub const m_flTrackAmount: usize = 0x1A98; //
        pub const m_flCollideRadius: usize = 0x1A9C; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_ProjectMindVData {
        pub const m_ProjectMindModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_SilenceProc_DebuffVData {
        pub const m_SilenceModifier: usize = 0x638; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_AcolytesGlove {
    }
    // Parent: CitadelItemVData
    // Field count: 3
    pub mod CCitadel_Item_BubbleVData {
        pub const m_CastParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastTargetSound: usize = 0x16B8; //
        pub const m_BubbleModifier: usize = 0x16C8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierHoldingGoldenIdolVData {
        pub const m_IdolParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: None
    // Field count: 6
    pub mod SequenceHistory_t {
        pub const m_hSequence: usize = 0x0; //
        pub const m_flSeqStartTime: usize = 0x4; //
        pub const m_flSeqFixedCycle: usize = 0x8; //
        pub const m_nSeqLoopMode: usize = 0xC; //
        pub const m_flPlaybackRate: usize = 0x10; //
        pub const m_flCyclesPerSecond: usize = 0x14; //
    }
    // Parent: C_PathParticleRope
    // Field count: 1
    pub mod C_CitadelZiplinePath {
        pub const m_iLaneNumber: usize = 0x670; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_BaseHeldItemVData {
        pub const m_ItemModel: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_Traveler_FireRate {
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CAbilityCadenceGrandFinaleVData {
        pub const m_StageModel: usize = 0x1590; //
        pub const m_flStageModelHeight: usize = 0x1670; //
        pub const m_flStageModelWidth: usize = 0x1674; //
        pub const m_flStageModelLength: usize = 0x1678; //
        pub const m_flStageModelScale: usize = 0x167C; //
        pub const m_GrandFinaleAOEModifier: usize = 0x1680; //
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 1
    pub mod CCitadel_Modifier_Gravity_Lasso_Enemy {
        pub const m_eHoldPosition: usize = 0xC8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 8
    pub mod CCitadel_Ability_Bull_Leap {
        pub const m_flBoostYaw: usize = 0xCA0; //
        pub const m_vecCrashPosition: usize = 0xCA4; //
        pub const m_vecCrashDirection: usize = 0xCB0; //
        pub const m_eLeapState: usize = 0xCBC; //
        pub const m_flStateEnterTime: usize = 0xCC0; //
        pub const m_flNextStateTime: usize = 0xCC8; //
        pub const m_flBoostEndTime: usize = 0xCE0; //
        pub const m_vecLastVel: usize = 0xE50; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Infuser {
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 1
    pub mod CCitadel_Ability_Tier2Boss_RocketBarrage {
        pub const m_nGrenadesLeft: usize = 0xCA0; //
    }
    // Parent: C_BaseEntity
    // Field count: 2
    pub mod CPointModifierThinker {
        pub const m_hModifier: usize = 0x560; //
        pub const m_bSendToClients: usize = 0x578; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 9
    pub mod C_EnvDecal {
        pub const m_hDecalMaterial: usize = 0x858; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_flWidth: usize = 0x860; //
        pub const m_flHeight: usize = 0x864; //
        pub const m_flDepth: usize = 0x868; //
        pub const m_nRenderOrder: usize = 0x86C; //
        pub const m_bProjectOnWorld: usize = 0x870; //
        pub const m_bProjectOnCharacters: usize = 0x871; //
        pub const m_bProjectOnWater: usize = 0x872; //
        pub const m_flDepthSortBias: usize = 0x874; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_Tornado_HoldInPlace {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityLockDownVData {
        pub const m_CastParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffModifier: usize = 0x1670; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CCitadel_Ability_NanoDash_VData {
        pub const m_DashModifier: usize = 0x1590; //
        pub const m_DashImpactEffect: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DashSwingEffect: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DashLineEffect: usize = 0x1760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SlashSwingEffect: usize = 0x1840; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDashStart: usize = 0x1920; //
        pub const m_strSlashStart: usize = 0x1930; //
        pub const m_strSlashImpactSound: usize = 0x1940; //
        pub const m_cameraSequenceSlash: usize = 0x1950; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Nano_CatForm {
        pub const m_ModelChange: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_NearbyEnemyBoostVData {
        pub const m_BerserkerSound: usize = 0x638; //
        pub const m_BuffModifier: usize = 0x648; //
    }
    // Parent: C_BaseEntity
    // Field count: 18
    pub mod C_ColorCorrection {
        pub const m_vecOrigin: usize = 0x560; //
        pub const m_MinFalloff: usize = 0x56C; //
        pub const m_MaxFalloff: usize = 0x570; //
        pub const m_flFadeInDuration: usize = 0x574; //
        pub const m_flFadeOutDuration: usize = 0x578; //
        pub const m_flMaxWeight: usize = 0x57C; //
        pub const m_flCurWeight: usize = 0x580; //
        pub const m_netlookupFilename: usize = 0x584; //
        pub const m_bEnabled: usize = 0x784; //
        pub const m_bMaster: usize = 0x785; //
        pub const m_bClientSide: usize = 0x786; //
        pub const m_bExclusive: usize = 0x787; //
        pub const m_bEnabledOnClient: usize = 0x788; //
        pub const m_flCurWeightOnClient: usize = 0x78C; //
        pub const m_bFadingIn: usize = 0x790; //
        pub const m_flFadeStartWeight: usize = 0x794; //
        pub const m_flFadeStartTime: usize = 0x798; //
        pub const m_flFadeDuration: usize = 0x79C; //
    }
    // Parent: C_BaseCombatCharacter
    // Field count: 3
    pub mod C_AI_BaseNPC {
        pub const m_NPCState: usize = 0xD18; //
        pub const m_bFadeCorpse: usize = 0xD1C; //
        pub const m_bImportantRagdoll: usize = 0xD1D; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Warden_RiotProtocol_EnemyDebuff {
        pub const m_flEnemyMoveSlow: usize = 0xC0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ProjectMind {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Bomber_ULT {
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_Upgrade_OverdriveClip_VData {
        pub const m_OverdriveClipModifier: usize = 0x15D8; //
        pub const m_ReloadModifier: usize = 0x15E8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_ChainLightningEffectVData {
        pub const m_ChainParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChainSound: usize = 0x718; //
        pub const m_VictimSound: usize = 0x728; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_Passive_Camouflage {
        pub const m_flRate: usize = 0xC0; //
        pub const m_vLastPosition: usize = 0xC4; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_ThermalDetonator_Thinker {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CAbility_Synth_Barrage {
        pub const m_nProjectilesScheduled: usize = 0xF08; //
        pub const m_ChannelParticle: usize = 0xF0C; //
        pub const m_flNextShootTime: usize = 0xF10; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityWreckerUltimateVData {
        pub const m_BeamParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChargeParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ActiveModifier: usize = 0x1750; //
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProc
    // Field count: 0
    pub mod CCitadel_Modifier_MedicBullets {
    }
    // Parent: CCitadelModifierVData
    // Field count: 8
    pub mod CCitadel_Modifier_BaseEventProcVData {
        pub const m_bProcChanceAffectedByEffectiveness: usize = 0x638; //
        pub const m_bShouldApplyAbilityCooldown: usize = 0x639; //
        pub const m_bCanProcMultipleTimesOnOneTarget: usize = 0x63A; //
        pub const m_bCanProcByOtherObjects: usize = 0x63B; //
        pub const m_nAbilityTargetTypes: usize = 0x63C; //
        pub const m_nAbilityTargetFlags: usize = 0x640; //
        pub const m_vecProcDamageTypes: usize = 0x648; // CUtlVector<ECitadelDamageType>
        pub const m_nRequiredDamageFlags: usize = 0x660; //
    }
    // Parent: CScaleFunctionVData
    // Field count: 1
    pub mod CScaleFunctionAbilityPropertyMultiStatsVData {
        pub const m_vecScalingStats: usize = 0x40; //
    }
    // Parent: None
    // Field count: 8
    pub mod CModelState {
        pub const m_hModel: usize = 0xD0; // CStrongHandle<InfoForResourceTypeCModel>
        pub const m_ModelName: usize = 0xD8; //
        pub const m_bClientClothCreationSuppressed: usize = 0x118; //
        pub const m_MeshGroupMask: usize = 0x1D0; //
        pub const m_nBodyGroupChoices: usize = 0x220; //
        pub const m_nIdealMotionType: usize = 0x26A; //
        pub const m_nForceLOD: usize = 0x26B; //
        pub const m_nClothUpdateFlags: usize = 0x26C; //
    }
    // Parent: CBaseModifierAura
    // Field count: 0
    pub mod CCitadelModifierAura {
    }
    // Parent: C_FuncBrush
    // Field count: 0
    pub mod C_CitadelSpawnBlocker {
    }
    // Parent: None
    // Field count: 1
    pub mod C_EconEntity__AttachedModelData_t {
        pub const m_iModelDisplayFlags: usize = 0x0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 5
    pub mod CCitadel_Ability_TurretClone {
        pub const m_flTurretExpireTime: usize = 0xDB8; //
        pub const m_bHasTurretReady: usize = 0xDC0; //
        pub const m_iCurrentSwapCount: usize = 0xDC4; //
        pub const m_vecTurretPosition: usize = 0xDC8; //
        pub const m_nTurretFXIndex: usize = 0xDD4; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_FireBeetles_Debuff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifierThumper_3VData {
        pub const m_DroneParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LoopSound: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityCadenceCrescendoVData {
        pub const m_CrescendoAOEModifier: usize = 0x1590; // CEmbeddedSubclass<CCitadelModifierAura>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_SleepBomb {
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_Thumper_EnemyPulled_VData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_UltCombo {
        pub const m_flLastAttackTime: usize = 0xCA0; //
        pub const m_nAttackNum: usize = 0xCA4; //
        pub const m_iBonusHealth: usize = 0xD50; //
        pub const m_hTarget: usize = 0xD54; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Rolling_FireBall {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Forge_MiniTurret_InnateModifier {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_PowerSurge {
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CModifier_Upgrade_ArcaneSurge_AbilityWatcher {
        pub const m_hBuffedAbility: usize = 0xC0; //
        pub const m_bRecastingBuffedAbility: usize = 0xC4; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_DivinersKevlarBuff {
    }
    // Parent: CTier3BossAbility
    // Field count: 0
    pub mod CCitadel_Ability_Tier3Boss_LaserBeam {
    }
    // Parent: CCitadelModifierVData
    // Field count: 11
    pub mod CCitadel_Modifier_InvisVData {
        pub const m_InvisLoopParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_InvisDetectRadiusParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_InvisRevealedParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flDesatFactor: usize = 0x8D8; //
        pub const m_strInvisRevealedSound: usize = 0x8E0; //
        pub const m_bFadeInsteadOfRemoveOnBulletFire: usize = 0x8F0; //
        pub const m_bFadeInsteadOfRemoveOnAbilityUse: usize = 0x8F1; //
        pub const m_bBreakOnItemUse: usize = 0x8F2; //
        pub const m_bFadeToVisibleAtEndOfDuration: usize = 0x8F3; //
        pub const m_flMinCloak: usize = 0x8F4; //
        pub const m_flMaxCloak: usize = 0x8F8; //
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 29
    pub mod CCitadel_BreakablePropVData {
        pub const m_bBreakOnDodgeTouch: usize = 0x28; //
        pub const m_bRenderAfterDeath: usize = 0x29; //
        pub const m_bSolidAfterDeath: usize = 0x2A; //
        pub const m_bIsPermanent: usize = 0x2B; //
        pub const m_bDamagedByBullets: usize = 0x2C; //
        pub const m_bDamagedByMelee: usize = 0x2D; //
        pub const m_bDamagedByAbilities: usize = 0x2E; //
        pub const m_hModel: usize = 0x30; //
        pub const m_sAnimgraphParamDamageReceived: usize = 0x110; //
        pub const m_sAnimgraphParamOnHit: usize = 0x118; //
        pub const m_sAnimgraphParamOnRespawn: usize = 0x120; //
        pub const m_sBreakSound: usize = 0x128; //
        pub const m_sDamageSound: usize = 0x138; //
        pub const m_sHeavyDamageSound: usize = 0x148; //
        pub const m_sHitIndicatorSound: usize = 0x158; //
        pub const m_iHealth: usize = 0x168; //
        pub const m_flInitialSpawnTime: usize = 0x16C; //
        pub const m_flRespawnTime: usize = 0x170; //
        pub const m_flInitialSpawnTimeTest: usize = 0x174; //
        pub const m_flRespawnTimeTest: usize = 0x178; //
        pub const m_bIsMantleable: usize = 0x17C; //
        pub const m_flPrimaryDropChance: usize = 0x180; //
        pub const m_eRollType: usize = 0x184; //
        pub const m_vecPrimaryPickups: usize = 0x188; // CUtlVector<BreakablePowerupDropDefinition_t>
        pub const m_iMatchTimeMinsForLevel2Pickups: usize = 0x1A0; //
        pub const m_vecPickups_lv2: usize = 0x1A8; // CUtlVector<BreakablePowerupDropDefinition_t>
        pub const m_iMatchTimeMinsForLevel3Pickups: usize = 0x1C0; //
        pub const m_vecPickups_lv3: usize = 0x1C8; // CUtlVector<BreakablePowerupDropDefinition_t>
        pub const m_iLootListDeckSize: usize = 0x1E0; //
    }
    // Parent: None
    // Field count: 2
    pub mod CAttributeList {
        pub const m_Attributes: usize = 0x8; //
        pub const m_pManager: usize = 0x70; //
    }
    // Parent: CCitadel_Ability_PrimaryWeaponVData
    // Field count: 1
    pub mod CAbilityCadencePrimaryWeaponVData {
        pub const m_DebuffModifier: usize = 0x15E0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_CrowdControl {
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 2
    pub mod CCitadel_Modifier_VandalSurge {
        pub const m_vecFloatDest: usize = 0x138; //
        pub const m_vecStartingPos: usize = 0x144; //
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_RestorativeGoo {
        pub const m_flEarliestBreakoutTime: usize = 0xC0; //
        pub const m_hGooCube: usize = 0x3A0; //
        pub const m_flBreakoutPercentage: usize = 0x3A4; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Upgrade_KineticSashTriggered_VData {
        pub const m_TriggeredSound: usize = 0x638; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Tech_BleedVData {
        pub const m_DamageParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_Item_SelfBuffModifierVData {
        pub const m_BuffModifier: usize = 0x15D8; //
    }
    // Parent: C_MultiplayRules
    // Field count: 0
    pub mod C_TeamplayRules {
    }
    // Parent: C_CitadelProjectile
    // Field count: 1
    pub mod C_CitadelBoomerangProjectile {
        pub const m_bReturning: usize = 0x8E0; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Upgrade_StabilizingTripod {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_RegeneratingBulletShield {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ItemPickupTimer {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Mirage_SandPhantom_Passive_Victim {
        pub const m_flLastProcTime: usize = 0xD0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Wrecker_BoulderGrenade {
        pub const m_hHitTroopers: usize = 0xCA0; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_nBallParticle: usize = 0xCBC; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CModifier_Upgrade_KineticSashTriggered {
        pub const m_nBonusClip: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Objective_HealthGrowth {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_DamageResistanceVData {
        pub const m_flDamageResistancePerSecond: usize = 0x638; //
        pub const m_flTickInterval: usize = 0x63C; //
        pub const m_flDamageResistanceBonusPerGameMinute: usize = 0x640; //
    }
    // Parent: C_BaseToggle
    // Field count: 1
    pub mod C_BaseDoor {
        pub const m_bIsUsable: usize = 0x858; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 1
    pub mod C_Citadel_Projectile_Bebop_Hook {
        pub const m_iChainEffect: usize = 0x8E0; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_GlassCannon {
        pub const m_nKillsEarned: usize = 0xCB8; //
    }
    // Parent: C_BaseClientUIEntity
    // Field count: 2
    pub mod C_PointClientUIDialog {
        pub const m_hActivator: usize = 0x888; //
        pub const m_bStartEnabled: usize = 0x88C; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Trapper_SpiderJar_VData {
        pub const m_SpiderExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_JarExplodeParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SpiritStealDebuffModifier: usize = 0x1750; //
    }
    // Parent: CCitadel_Modifier_StatStealBaseVData
    // Field count: 1
    pub mod CCitadel_Modifier_Mirage_FireScarabs_WatcherVData {
        pub const m_HealModifier: usize = 0x658; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityTargetdummy1VData {
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CAbilityRollingFireBallVData {
        pub const m_flBallLifetime: usize = 0x1590; //
        pub const m_flBallStepUpHeight: usize = 0x1594; //
        pub const m_flBallDistAboveGround: usize = 0x1598; //
        pub const m_flBallFloatDownRate: usize = 0x159C; //
        pub const m_flBallSpeed: usize = 0x15A0; //
        pub const m_flBallTraceRadius: usize = 0x15A4; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_Hook {
        pub const m_hHookVictim: usize = 0xCA0; //
        pub const m_hProjectile: usize = 0xCA4; //
        pub const m_vecHookTargetStartPos: usize = 0xCA8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_VoidSphere {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Bull_Heal {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_WeaponPowerForHealthVData {
        pub const m_ActiveBuff: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SuperNeutralChargeActive {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_InvisFading {
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_Rutger_Pulse_Aura_VData {
        pub const m_empWaveParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_CProjectile_Rutger_Rocket {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ControlPointBlockerAuraTarget {
    }
    // Parent: C_PointEntity
    // Field count: 1
    pub mod CPointChildModifier {
        pub const m_bOrphanInsteadOfDeletingChildrenOnRemove: usize = 0x560; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Synth_PlasmaFlux_WeaponDamage_VData {
        pub const m_BuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ShieldedSentry {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_AblativeCoatResistBuffVData {
        pub const m_ResistBuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_ReloadSpeed {
        pub const m_flReloadSpeed: usize = 0xC0; //
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod C_InfoLadderDismount {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_TechCleave {
    }
    // Parent: C_BaseModelEntity
    // Field count: 13
    pub mod CCitadelZipLineNode {
        pub const m_vecConnections: usize = 0x898; //
        pub const m_eCaptureState: usize = 0x8B0; //
        pub const m_iPrimaryLane: usize = 0x8B2; //
        pub const m_nRopesParity: usize = 0x8B4; //
        pub const m_bCornerNode: usize = 0x8B6; //
        pub const m_bCapturable: usize = 0x8B7; //
        pub const m_bAlwaysUsable: usize = 0x8B8; //
        pub const m_bOneWay: usize = 0x8B9; //
        pub const m_bDisableZippingToByPlayers: usize = 0x8BA; //
        pub const m_bUseForMinimapDrawing: usize = 0x8BB; //
        pub const m_hGuardingBoss: usize = 0x8BC; //
        pub const m_flRopeRadius: usize = 0x8C0; //
        pub const m_bEnabled: usize = 0x8C4; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CBaseModifierAura {
        pub const m_hAuraUnits: usize = 0xC0; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_flOverrideRadius: usize = 0xD8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbility_Synth_Affliction_VData {
        pub const m_DebuffModifier: usize = 0x1590; //
        pub const m_AoEParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 7
    pub mod CCitadel_Ability_Tokamak_HeatSinks_Inherent {
        pub const m_nIntervalsElapsed: usize = 0xCA0; //
        pub const m_NextShotTime: usize = 0xCA4; //
        pub const m_flDissipationRate: usize = 0xCA8; //
        pub const m_flDissipationTime: usize = 0xCAC; //
        pub const m_flHeatTime: usize = 0xCB0; //
        pub const m_flOverheatSoundTime: usize = 0xCB4; //
        pub const m_bOverheating: usize = 0xCB8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_Viper_Ability04VData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PetrifyModifier: usize = 0x1670; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_VoidSphereBuffVData {
        pub const m_RapidFireParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Surging_Power {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_CQC_ProcVData {
        pub const m_DebuffModifier: usize = 0x668; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 6
    pub mod CCitadel_Modifier_Tier3_DamagePulseVData {
        pub const m_TargetParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strPulseTickSound: usize = 0x718; //
        pub const m_iMaxTargets: usize = 0x728; //
        pub const m_flRadius: usize = 0x72C; //
        pub const m_flDamagePerPulse: usize = 0x730; //
        pub const m_flTickRate: usize = 0x734; //
    }
    // Parent: CEntityComponent
    // Field count: 5
    pub mod CRenderComponent {
        pub const __m_pChainEntity: usize = 0x10; //
        pub const m_bIsRenderingWithViewModels: usize = 0x50; //
        pub const m_nSplitscreenFlags: usize = 0x54; //
        pub const m_bEnableRendering: usize = 0x60; //
        pub const m_bInterpolationReadyToDraw: usize = 0xB0; //
    }
    // Parent: CEnvSoundscape
    // Field count: 1
    pub mod CEnvSoundscapeProxy {
        pub const m_MainSoundscapeName: usize = 0x600; //
    }
    // Parent: C_SoundEventEntity
    // Field count: 2
    pub mod C_SoundEventOBBEntity {
        pub const m_vMins: usize = 0x620; //
        pub const m_vMaxs: usize = 0x62C; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 7
    pub mod CCitadel_Ability_Tokamak_HotShot {
        pub const m_flDPS: usize = 0xE18; //
        pub const m_flNextDamageTick: usize = 0xE1C; //
        pub const m_vStart: usize = 0xE20; //
        pub const m_vEnd: usize = 0xE2C; //
        pub const m_vecEntitiesHit: usize = 0xE38; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_angBeamAngles: usize = 0xE50; //
        pub const m_bNeedsBeamReset: usize = 0xE68; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_WreckerScrapBlastDebuffVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 6
    pub mod CCitadel_Modifier_IceDome {
        pub const m_hBlocker: usize = 0xC0; //
        pub const m_hFriendlyAura: usize = 0xC4; //
        pub const m_hEnemyAura: usize = 0xC8; //
        pub const m_nParticleIndex: usize = 0xCC; //
        pub const m_flStartTime: usize = 0xD0; //
        pub const m_vOrigin: usize = 0x1B8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Parry {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CItemAOESilenceModifierVData {
        pub const m_strSilenceTargetSound: usize = 0x638; //
        pub const m_SilenceModifier: usize = 0x648; //
    }
    // Parent: CitadelItemVData
    // Field count: 3
    pub mod CCitadel_UtilityUpgrade_AOESmokeBombVData {
        pub const m_CastCompleteParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strBuffGainedSound: usize = 0x16B8; //
        pub const m_InvisModifier: usize = 0x16C8; //
    }
    // Parent: CitadelItemVData
    // Field count: 10
    pub mod CItem_WarpStone_VData {
        pub const m_CasterModifier: usize = 0x15D8; //
        pub const m_CasterDebuffModifier: usize = 0x15E8; //
        pub const m_strExplodeSound: usize = 0x15F8; //
        pub const m_CastDelayParticle: usize = 0x1608; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportTrailParticle: usize = 0x16E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flGroundProbeSpeed: usize = 0x17C8; //
        pub const m_flGroundStepDown: usize = 0x17CC; //
        pub const m_flGroundStepUp: usize = 0x17D0; //
        pub const m_iMaxGroundIterations: usize = 0x17D4; //
        pub const m_flVelocityScale: usize = 0x17D8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_SuperAcolytesGlove {
    }
    // Parent: None
    // Field count: 8
    pub mod LockonTarget_t {
        pub const m_flGainRate: usize = 0x30; //
        pub const m_flDrainRate: usize = 0x34; //
        pub const m_flMaxValue: usize = 0x38; //
        pub const m_nPrevFullStacks: usize = 0x3C; //
        pub const m_flLatchedValue: usize = 0x40; //
        pub const m_flLatchedTime: usize = 0x44; //
        pub const m_eLockonState: usize = 0x48; //
        pub const m_hTarget: usize = 0x4C; //
    }
    // Parent: CEntityComponent
    // Field count: 66
    pub mod CLightComponent {
        pub const __m_pChainEntity: usize = 0x38; //
        pub const m_Color: usize = 0x75; //
        pub const m_SecondaryColor: usize = 0x79; //
        pub const m_flBrightness: usize = 0x80; //
        pub const m_flBrightnessScale: usize = 0x84; //
        pub const m_flBrightnessMult: usize = 0x88; //
        pub const m_flRange: usize = 0x8C; //
        pub const m_flFalloff: usize = 0x90; //
        pub const m_flAttenuation0: usize = 0x94; //
        pub const m_flAttenuation1: usize = 0x98; //
        pub const m_flAttenuation2: usize = 0x9C; //
        pub const m_flTheta: usize = 0xA0; //
        pub const m_flPhi: usize = 0xA4; //
        pub const m_hLightCookie: usize = 0xA8; //
        pub const m_nCascades: usize = 0xB0; //
        pub const m_nCastShadows: usize = 0xB4; //
        pub const m_nShadowWidth: usize = 0xB8; //
        pub const m_nShadowHeight: usize = 0xBC; //
        pub const m_bRenderDiffuse: usize = 0xC0; //
        pub const m_nRenderSpecular: usize = 0xC4; //
        pub const m_bRenderTransmissive: usize = 0xC8; //
        pub const m_flOrthoLightWidth: usize = 0xCC; //
        pub const m_flOrthoLightHeight: usize = 0xD0; //
        pub const m_nStyle: usize = 0xD4; //
        pub const m_Pattern: usize = 0xD8; //
        pub const m_nCascadeRenderStaticObjects: usize = 0xE0; //
        pub const m_flShadowCascadeCrossFade: usize = 0xE4; //
        pub const m_flShadowCascadeDistanceFade: usize = 0xE8; //
        pub const m_flShadowCascadeDistance0: usize = 0xEC; //
        pub const m_flShadowCascadeDistance1: usize = 0xF0; //
        pub const m_flShadowCascadeDistance2: usize = 0xF4; //
        pub const m_flShadowCascadeDistance3: usize = 0xF8; //
        pub const m_nShadowCascadeResolution0: usize = 0xFC; //
        pub const m_nShadowCascadeResolution1: usize = 0x100; //
        pub const m_nShadowCascadeResolution2: usize = 0x104; //
        pub const m_nShadowCascadeResolution3: usize = 0x108; //
        pub const m_bUsesBakedShadowing: usize = 0x10C; //
        pub const m_nShadowPriority: usize = 0x110; //
        pub const m_nBakedShadowIndex: usize = 0x114; //
        pub const m_bRenderToCubemaps: usize = 0x118; //
        pub const m_nDirectLight: usize = 0x11C; //
        pub const m_nIndirectLight: usize = 0x120; //
        pub const m_flFadeMinDist: usize = 0x124; //
        pub const m_flFadeMaxDist: usize = 0x128; //
        pub const m_flShadowFadeMinDist: usize = 0x12C; //
        pub const m_flShadowFadeMaxDist: usize = 0x130; //
        pub const m_bEnabled: usize = 0x134; //
        pub const m_bFlicker: usize = 0x135; //
        pub const m_bPrecomputedFieldsValid: usize = 0x136; //
        pub const m_vPrecomputedBoundsMins: usize = 0x138; //
        pub const m_vPrecomputedBoundsMaxs: usize = 0x144; //
        pub const m_vPrecomputedOBBOrigin: usize = 0x150; //
        pub const m_vPrecomputedOBBAngles: usize = 0x15C; //
        pub const m_vPrecomputedOBBExtent: usize = 0x168; //
        pub const m_flPrecomputedMaxRange: usize = 0x174; //
        pub const m_nFogLightingMode: usize = 0x178; //
        pub const m_flFogContributionStength: usize = 0x17C; //
        pub const m_flNearClipPlane: usize = 0x180; //
        pub const m_SkyColor: usize = 0x184; //
        pub const m_flSkyIntensity: usize = 0x188; //
        pub const m_SkyAmbientBounce: usize = 0x18C; //
        pub const m_bUseSecondaryColor: usize = 0x190; //
        pub const m_bMixedShadows: usize = 0x191; //
        pub const m_flLightStyleStartTime: usize = 0x194; //
        pub const m_flCapsuleLength: usize = 0x198; //
        pub const m_flMinRoughness: usize = 0x19C; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_GooGrenade {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_IcePath_Buff {
    }
    // Parent: CCitadelModifier
    // Field count: 7
    pub mod CCitadel_Modifier_ProjectMind {
        pub const m_particleStart: usize = 0xC0; //
        pub const m_particleEnd: usize = 0xC4; //
        pub const m_particleTrail: usize = 0xC8; //
        pub const m_vecEndLocation: usize = 0xCC; //
        pub const m_vecStartPosition: usize = 0xD8; //
        pub const m_flStartDelay: usize = 0xE4; //
        pub const m_vecApplyOffset: usize = 0xE8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_MetalSkinVData {
        pub const m_BuffStartParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffEndParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strHitProcSound: usize = 0x7F8; //
    }
    // Parent: CTier3BossAbility
    // Field count: 0
    pub mod CCitadel_Ability_Tier3Boss_DamagePulse {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Core_PushTarget {
    }
    // Parent: None
    // Field count: 0
    pub mod CEntityComponent {
    }
    // Parent: IEconItemInterface
    // Field count: 16
    pub mod C_EconItemView {
        pub const m_iItemDefinitionIndex: usize = 0x8; //
        pub const m_iEntityQuality: usize = 0xC; //
        pub const m_iEntityLevel: usize = 0x10; //
        pub const m_iItemID: usize = 0x18; //
        pub const m_iAccountID: usize = 0x20; //
        pub const m_iInventoryPosition: usize = 0x24; //
        pub const m_bInitialized: usize = 0x30; //
        pub const m_nOverrideStyle: usize = 0x31; //
        pub const m_bIsStoreItem: usize = 0x32; //
        pub const m_bIsTradeItem: usize = 0x33; //
        pub const m_bHasComputedAttachedParticles: usize = 0x34; //
        pub const m_bHasAttachedParticles: usize = 0x35; //
        pub const m_iEntityQuantity: usize = 0x38; //
        pub const m_unClientFlags: usize = 0x3C; //
        pub const m_unOverrideOrigin: usize = 0x40; //
        pub const m_AttributeList: usize = 0x58; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 0
    pub mod CCitadelModifer_Viscous_Goo_Aura_VData {
    }
    // Parent: C_BaseEntity
    // Field count: 2
    pub mod C_HandleTest {
        pub const m_Handle: usize = 0x560; //
        pub const m_bSendHandle: usize = 0x564; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CAbilityTokamakHotShotVData {
        pub const m_LaserModifier: usize = 0x1590; //
        pub const m_strLaserStartSound: usize = 0x15A0; //
        pub const m_strLaserEndSound: usize = 0x15B0; //
        pub const m_strLaserLoopSound: usize = 0x15C0; //
        pub const m_strLaserHitSound: usize = 0x15D0; //
        pub const m_ChargeParticle: usize = 0x15E0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BeamParticle: usize = 0x16C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x17A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_GroundParticle: usize = 0x1880; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_LifeDrain {
        pub const m_tDrainLifeStopTime: usize = 0xCA0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_StormCloud {
        pub const m_nTargetingParticleIndex: usize = 0xCA0; //
        pub const m_flFloat: usize = 0xE30; //
        pub const m_nLightningStrikesRemaining: usize = 0xE34; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 4
    pub mod CCitadel_Modifier_SuperAcolytesGlove_VData {
        pub const m_DebuffModifier: usize = 0x668; //
        pub const m_SwingParticle: usize = 0x678; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x758; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FistReadyEffect: usize = 0x838; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_BaseEntity
    // Field count: 1
    pub mod C_EnvWindClientside {
        pub const m_EnvWindShared: usize = 0x560; //
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod FilterDamageType {
        pub const m_iDamageType: usize = 0x5B8; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Rutger_ForceField_Aura {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Operative_Blindside_EnemyDebuff {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Chrono_PulseGrenade {
        pub const m_vLaunchPosition: usize = 0xCA0; //
        pub const m_qLaunchAngle: usize = 0xCAC; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Hornet_Sting {
        pub const m_flLastTickTime: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_VeilWalkerWatcher {
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 3
    pub mod CModifierNikumanVData {
        pub const m_SelfParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AOEFriendParticle: usize = 0x760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAmbientLoopingLocalPlayerSound: usize = 0x840; //
    }
    // Parent: C_PointClientUIWorldPanel
    // Field count: 1
    pub mod C_PointClientUIWorldTextPanel {
        pub const m_messageText: usize = 0xAC0; //
    }
    // Parent: CCitadel_Modifier_InvisVData
    // Field count: 4
    pub mod CCitadel_Modifier_LurkersAmbush_InvisVData {
        pub const m_flMaxCameraAngleForSeeing: usize = 0x900; //
        pub const m_flMaxDistanceForSeeing: usize = 0x904; //
        pub const m_flInvisBias: usize = 0x908; //
        pub const m_flSpottedMinTimeToStart: usize = 0x90C; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Thumper_4 {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_SilenceContraptions {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_Burrow {
        pub const m_bInGround: usize = 0xD80; //
        pub const m_SpinEndTime: usize = 0xD84; //
        pub const m_nBurrowEffect: usize = 0xD88; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Citadel_Bull_Leap_LandingBonuses {
    }
    // Parent: CCitadel_Item_TrackingProjectileApplyModifierVData
    // Field count: 2
    pub mod CCitadel_Item_Disarm_VData {
        pub const m_BuffModifier: usize = 0x16D8; //
        pub const m_DebuffModifier: usize = 0x16E8; //
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityProperty_BaseWeaponDamage {
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityProperty_TechDuration {
    }
    // Parent: C_PointClientUIWorldPanel
    // Field count: 0
    pub mod CUnitStatusOverlay {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_WeaponEater {
        pub const m_nWeaponPower: usize = 0xD98; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_FireRateAura {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_ComboBreakerHeal {
        pub const m_flAmountPerSecond: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_BonusDamagePercent {
    }
    // Parent: None
    // Field count: 5
    pub mod AbilityResource_t {
        pub const m_flCurrentValue: usize = 0x8; //
        pub const m_flPrevRegenRate: usize = 0xC; //
        pub const m_flMaxValue: usize = 0x10; //
        pub const m_flLatchTime: usize = 0x14; //
        pub const m_flLatchValue: usize = 0x18; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_Cadence_AnthemAOEVData {
        pub const m_AuraParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_BulletArmorReductionAura {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_Hornet_Snipe {
        pub const m_flScopeStartTime: usize = 0xF54; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_HornetMark {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_FlameDashBurnVData {
        pub const m_DebuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Near_Climbable_Rope {
    }
    // Parent: CCitadelModifierVData
    // Field count: 10
    pub mod CCitadel_Modifier_Backdoor_ProtectionVData {
        pub const m_flActivationTime: usize = 0x638; //
        pub const m_flBackdoorProtectionDamageMitigationFromPlayers: usize = 0x63C; //
        pub const m_flHealthPerSecondRegen: usize = 0x640; //
        pub const m_flOutOfCombatHealthRegen: usize = 0x644; //
        pub const m_flOutOfCombatRegenDelay: usize = 0x648; //
        pub const m_flEffectsLingerTime: usize = 0x64C; //
        pub const m_ShieldImpactParticle: usize = 0x650; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShieldActiveParticle: usize = 0x730; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strActiveEffectConfigName: usize = 0x810; //
        pub const flShieldImpactDirectionOffset: usize = 0x818; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_RootVData {
    }
    // Parent: CPlayerPawnComponent
    // Field count: 15
    pub mod CPlayer_MovementServices {
        pub const m_nImpulse: usize = 0x40; //
        pub const m_nButtons: usize = 0x48; //
        pub const m_nQueuedButtonDownMask: usize = 0x68; //
        pub const m_nQueuedButtonChangeMask: usize = 0x70; //
        pub const m_nButtonDoublePressed: usize = 0x78; //
        pub const m_pButtonPressedCmdNumber: usize = 0x80; //
        pub const m_nLastCommandNumberProcessed: usize = 0x180; //
        pub const m_nToggleButtonDownMask: usize = 0x188; //
        pub const m_flMaxspeed: usize = 0x198; //
        pub const m_arrForceSubtickMoveWhen: usize = 0x19C; //
        pub const m_flForwardMove: usize = 0x1AC; //
        pub const m_flLeftMove: usize = 0x1B0; //
        pub const m_flUpMove: usize = 0x1B4; //
        pub const m_vecLastMovementImpulses: usize = 0x1B8; //
        pub const m_vecOldViewAngles: usize = 0x1C4; //
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 5
    pub mod CModifierKnockdownVData {
        pub const m_flSatVolumeRadius: usize = 0x718; //
        pub const m_flSatVolumeFadeOut: usize = 0x71C; //
        pub const m_flGravityScale: usize = 0x720; //
        pub const m_flGetUpSeqDuration: usize = 0x724; //
        pub const m_cameraSequenceGetUp: usize = 0x728; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Mirage_FireBeetles_Debuff_VData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TangoTether_TetherReceiver {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_IncendiaryThinkerVData {
        pub const m_GroundParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Tier3Boss_Base {
    }
    // Parent: C_BaseEntity
    // Field count: 11
    pub mod CEnvSoundscape {
        pub const m_OnPlay: usize = 0x560; //
        pub const m_flRadius: usize = 0x588; //
        pub const m_soundEventName: usize = 0x590; //
        pub const m_bOverrideWithEvent: usize = 0x598; //
        pub const m_soundscapeIndex: usize = 0x59C; //
        pub const m_soundscapeEntityListId: usize = 0x5A0; //
        pub const m_positionNames: usize = 0x5A8; //
        pub const m_hProxySoundscape: usize = 0x5E8; //
        pub const m_bDisabled: usize = 0x5EC; //
        pub const m_soundscapeName: usize = 0x5F0; //
        pub const m_soundEventHash: usize = 0x5F8; //
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 15
    pub mod CBasePlayerVData {
        pub const m_sModelName: usize = 0x28; //
        pub const m_vecIntrinsicModifiers: usize = 0x108; //
        pub const m_flHeadDamageMultiplier: usize = 0x120; //
        pub const m_flChestDamageMultiplier: usize = 0x130; //
        pub const m_flStomachDamageMultiplier: usize = 0x140; //
        pub const m_flArmDamageMultiplier: usize = 0x150; //
        pub const m_flLegDamageMultiplier: usize = 0x160; //
        pub const m_flHoldBreathTime: usize = 0x170; //
        pub const m_flDrowningDamageInterval: usize = 0x174; //
        pub const m_nDrowningDamageInitial: usize = 0x178; //
        pub const m_nDrowningDamageMax: usize = 0x17C; //
        pub const m_nWaterSpeed: usize = 0x180; //
        pub const m_flUseRange: usize = 0x184; //
        pub const m_flUseAngleTolerance: usize = 0x188; //
        pub const m_flCrouchTime: usize = 0x18C; //
    }
    // Parent: CCitadelBaseShivAbility
    // Field count: 8
    pub mod CCitadel_Ability_Shiv_KillingBlow {
        pub const m_bActive: usize = 0xE60; //
        pub const m_hCurrentTarget: usize = 0xE64; //
        pub const m_vStartPosition: usize = 0xE68; //
        pub const m_vDeparturePosition: usize = 0xE74; //
        pub const m_flDepartureTime: usize = 0xE80; //
        pub const m_flArrivalTime: usize = 0xE98; //
        pub const m_vLastKnownSafePos: usize = 0xEB0; //
        pub const m_flDrainSuppressEndTime: usize = 0xEC0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityWreckerSalvageVData {
        pub const m_SalvageEnemyModifier: usize = 0x1590; //
        pub const m_StunEnemyModifier: usize = 0x15A0; //
        pub const m_BuffModifier: usize = 0x15B0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_Chrono_TimeWall_EffectVData {
        pub const m_DebuffModifier: usize = 0x638; //
        pub const m_BuffParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffParticle: usize = 0x728; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDamageSound: usize = 0x808; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_TechBleed_ProcVData {
        pub const m_BleedModifier: usize = 0x668; //
        pub const m_SlowModifier: usize = 0x678; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_RocketBarrage {
        pub const m_flCurrentTimeScale: usize = 0xE60; //
        pub const m_vecAimPos: usize = 0xE64; //
        pub const m_vecAimVel: usize = 0xE70; //
        pub const m_flLastUpdateTime: usize = 0xE7C; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityBloodShardsVData {
        pub const m_DebuffModifier: usize = 0x1590; //
        pub const m_ImpactParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Upgrade_KineticSash_VData {
        pub const m_KineticSashTriggeredModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_ClimbRopeSpeedVData {
        pub const m_flRampUpTime: usize = 0x638; //
        pub const m_flPercentageMultiplierStart: usize = 0x63C; //
        pub const m_flPercentageMultiplierEnd: usize = 0x640; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_DummyUnit {
    }
    // Parent: CEntityComponent
    // Field count: 14
    pub mod CCitadelAbilityComponent {
        pub const m_vecAbilities: usize = 0x70; //
        pub const m_vecUniversalItems: usize = 0x88; // C_NetworkUtlVectorBase<CUtlStringToken>
        pub const m_arPendingAsyncAbilityReservationSlots: usize = 0xA0; //
        pub const m_arPendingAsyncAbilityReservationAbilityIDs: usize = 0xB8; //
        pub const m_hSelectedAbility: usize = 0xD0; //
        pub const m_hPreviouslySelectedAbility: usize = 0xD4; //
        pub const m_bPreviousAbilityQueued: usize = 0xD8; //
        pub const m_flTimeScale: usize = 0xDC; //
        pub const m_flParticleTimeScale: usize = 0xE0; //
        pub const m_bInInterruptState: usize = 0xE4; //
        pub const m_ResourceStamina: usize = 0xE8; //
        pub const m_ResourceAbility: usize = 0x108; //
        pub const m_nExecuteAbilityMask: usize = 0x170; //
        pub const m_bSelectedEffectsStarted: usize = 0x178; //
    }
    // Parent: None
    // Field count: 2
    pub mod C_EnvWindShared__WindVariationEvent_t {
        pub const m_flWindAngleVariation: usize = 0x0; //
        pub const m_flWindSpeedVariation: usize = 0x4; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_InstantReload {
        pub const m_bIsManualReloading: usize = 0xCB8; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadelModifierAura_Cone {
    }
    // Parent: C_Sprite
    // Field count: 0
    pub mod CSpriteOriented {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityTokamakBreachVData {
        pub const m_AllySmokeAOEModifier: usize = 0x1590; //
        pub const m_EnemySmokeAOEModifier: usize = 0x15A0; //
        pub const m_PurgeParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityTokamakHeatSinksVData {
        pub const m_HeatDotModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_GrandFinale_Buff {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_ThrownShiv_Damage_Debuff {
        pub const m_nNumTicksRemaining: usize = 0xC0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_FlameDashVData {
        pub const m_FlameDashModifier: usize = 0x1590; //
        pub const m_DashBurstSound: usize = 0x15A0; //
        pub const m_ChargeHitSound: usize = 0x15B0; //
        pub const m_cameraSpeedBoost: usize = 0x15C0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CCitadel_Ability_PrimaryWeaponVData {
        pub const m_DOFWhileZoomed: usize = 0x1590; //
        pub const m_bDOFFarSettingsAreOffsetByGunRange: usize = 0x15A0; //
        pub const m_sDisarmedSound: usize = 0x15A8; //
        pub const m_flMinDisarmedSoundInterval: usize = 0x15B8; //
        pub const m_sObstructedShotSound: usize = 0x15C0; //
        pub const m_flActionReloadTimingStart: usize = 0x15D0; //
        pub const m_flActionReloadTimingDuration: usize = 0x15D4; //
        pub const m_strCrosshairCSSClass: usize = 0x15D8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_Tech_Defender_Shredders_Proc {
    }
    // Parent: CCitadel_Modifier_Out_Of_Combat_Health_Regen
    // Field count: 1
    pub mod CCitadel_Modifier_Apex_Watcher {
        pub const m_bShouldEnableBuff: usize = 0x138; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_ColdFrontAOE_VData {
        pub const m_TargetModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_ReloadSpeedVData {
        pub const m_flReloadSpeedPercent: usize = 0x638; //
        pub const m_bDestroyAfterReload: usize = 0x63C; //
    }
    // Parent: C_ParticleSystem
    // Field count: 5
    pub mod C_EnvParticleGlow {
        pub const m_flAlphaScale: usize = 0xE08; //
        pub const m_flRadiusScale: usize = 0xE0C; //
        pub const m_flSelfIllumScale: usize = 0xE10; //
        pub const m_ColorTint: usize = 0xE14; //
        pub const m_hTextureOverride: usize = 0xE18; //
    }
    // Parent: C_BaseEntity
    // Field count: 15
    pub mod C_SoundEventEntity {
        pub const m_bStartOnSpawn: usize = 0x560; //
        pub const m_bToLocalPlayer: usize = 0x561; //
        pub const m_bStopOnNew: usize = 0x562; //
        pub const m_bSaveRestore: usize = 0x563; //
        pub const m_bSavedIsPlaying: usize = 0x564; //
        pub const m_flSavedElapsedTime: usize = 0x568; //
        pub const m_iszSourceEntityName: usize = 0x570; //
        pub const m_iszAttachmentName: usize = 0x578; //
        pub const m_onGUIDChanged: usize = 0x580; //
        pub const m_onSoundFinished: usize = 0x5A8; //
        pub const m_flClientCullRadius: usize = 0x5D0; //
        pub const m_iszSoundName: usize = 0x600; //
        pub const m_hSource: usize = 0x610; //
        pub const m_nEntityIndexSelection: usize = 0x614; //
        pub const m_bClientSideOnly: usize = 0x0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Operative_Blindside {
        pub const m_vLaunchPosition: usize = 0xCA0; //
        pub const m_qLaunchAngle: usize = 0xCAC; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_SettingSun {
        pub const m_TargetPreviews: usize = 0xCA0; //
        pub const m_bWasSelected: usize = 0xD30; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SilenceProc_Immunity {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_RegenerativeArmorVData {
        pub const m_RegenModifier: usize = 0x15D8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 1
    pub mod CCitadel_Modifier_QuickSilver_Watcher {
        pub const m_bProcNextHit: usize = 0x210; //
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_BaseEventProc {
        pub const m_vecProcdUnitsThisShot: usize = 0xC0; //
        pub const m_vecTrackedUnitsThisFrame: usize = 0xD8; //
        pub const m_nLastShotId: usize = 0xF0; //
    }
    // Parent: CAI_BaseNPCVData
    // Field count: 49
    pub mod CAI_CitadelNPCVData {
        pub const m_sAG2VariationName: usize = 0x3A8; //
        pub const m_mapBoundAbilities: usize = 0x488; //
        pub const m_flSightRangePlayers: usize = 0x4B0; //
        pub const m_flSightRangeNPCs: usize = 0x4B4; //
        pub const m_MeleeAnimName: usize = 0x4B8; //
        pub const m_flMeleeAttemptRange: usize = 0x4C0; //
        pub const m_flMeleeHitRange: usize = 0x4C4; //
        pub const m_MeleeAttackPoints: usize = 0x4C8; //
        pub const m_flMaxHealthBarDrawDistance: usize = 0x4E0; //
        pub const m_flWalkSpeed: usize = 0x4E4; //
        pub const m_flRunSpeed: usize = 0x4E8; //
        pub const m_flTurnRate: usize = 0x4EC; //
        pub const m_flAcceleration: usize = 0x4F0; //
        pub const m_flStepHeight: usize = 0x4F4; //
        pub const m_navHull: usize = 0x4F8; //
        pub const m_BeamStartSound: usize = 0x500; //
        pub const m_BeamStopSound: usize = 0x510; //
        pub const m_BeamPointStartLoopSound: usize = 0x520; //
        pub const m_BeamPointEndLoopSound: usize = 0x530; //
        pub const m_BeamPointClosestLoopSound: usize = 0x540; //
        pub const m_strAmbientLoopSound: usize = 0x550; //
        pub const m_DeathSound: usize = 0x560; //
        pub const m_strLastHitSound: usize = 0x570; //
        pub const m_bPlayLastHitSound: usize = 0x580; //
        pub const m_MeleeHitSound: usize = 0x588; //
        pub const m_MeleeHitPlayerSound: usize = 0x598; //
        pub const m_sDefaultMaterialGroupName: usize = 0x5A8; //
        pub const m_sEnemyMaterialGroupName: usize = 0x5B0; //
        pub const m_sTeam1MaterialGroupName: usize = 0x5B8; //
        pub const m_sTeam2MaterialGroupName: usize = 0x5C0; //
        pub const m_MeleeSwingParticle: usize = 0x5C8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_MeleeActivateParticle: usize = 0x6A8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flModelScale: usize = 0x788; //
        pub const m_DeathParticle: usize = 0x790; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HealthBarParticle: usize = 0x870; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sHealthBarAttachment: usize = 0x950; //
        pub const m_HealthBarColorFriend: usize = 0x958; //
        pub const m_HealthBarColorEnemy: usize = 0x95C; //
        pub const m_HealthBarColorTeam1: usize = 0x960; //
        pub const m_HealthBarColorTeam2: usize = 0x964; //
        pub const m_HealthBarColorTeamNeutral: usize = 0x968; //
        pub const m_flHealthBarOffset: usize = 0x96C; //
        pub const m_bSpawnBreakablesOnDeath: usize = 0x970; //
        pub const m_flBreakableForceScale: usize = 0x974; //
        pub const m_flPhysicsImpulseMultiplier: usize = 0x978; //
        pub const m_flBeamWeaponWidth: usize = 0x97C; //
        pub const m_flBeamTurnRate: usize = 0x980; //
        pub const m_BeamWeaponParticle: usize = 0x988; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WeaponInfo: usize = 0xA68; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_Cadence_Crescendo_AOE_VData {
        pub const m_AuraParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilitySpiderShieldVData {
        pub const m_BuffModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Burrow {
        pub const m_flLastDamageTime: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifierLashGrappleTargetVData {
        pub const m_LockingOnParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LockedOnParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_HornetMark {
        pub const m_nFXIndex: usize = 0xCA0; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_UtilityUpgrade_HealthNova_VData {
        pub const m_HealingModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_DebuffImmunityVData {
        pub const m_ShieldParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PlayerShieldParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CNPC_SimpleAnimatingAIVData
    // Field count: 20
    pub mod CNPC_ShieldedSentryVData {
        pub const m_flZShootPostionOffset: usize = 0x108; //
        pub const m_LaserSightParticle: usize = 0x110; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_KillExplosionParticle: usize = 0x1F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AutoDestructParticle: usize = 0x2D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DeployProgressModifier: usize = 0x3B0; //
        pub const m_NearDeathModifier: usize = 0x3C0; //
        pub const m_IntrinsicModifier: usize = 0x3D0; //
        pub const m_sSpawnSound: usize = 0x3E0; //
        pub const m_sKillExplosionSound: usize = 0x3F0; //
        pub const m_sTargetAcquiredLocalSound: usize = 0x400; //
        pub const m_sTargetAcquiredSound: usize = 0x410; //
        pub const m_flIdleTurnSpeed: usize = 0x420; //
        pub const m_flIdleTurnAngles: usize = 0x424; //
        pub const m_flTrooperTakeDamageMult: usize = 0x428; //
        pub const m_flNeutralTakeDamageMulti: usize = 0x42C; //
        pub const m_flNotifyEventTime: usize = 0x430; //
        pub const m_flNearDeathDuration: usize = 0x434; //
        pub const m_flMinimapRevealTime: usize = 0x438; //
        pub const m_flMinLifetime: usize = 0x43C; //
        pub const m_flAttackThinkTime: usize = 0x440; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Operative_Revelation_Caster {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_SleepDaggerAsleepVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PostSleepModifier: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 11
    pub mod CAbilityImmobilizeTrapVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PreviewRingParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TrapHighlightParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ArmedParticle: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strTripSound: usize = 0x1910; //
        pub const m_strExplodeSound: usize = 0x1920; //
        pub const m_strExpiredSound: usize = 0x1930; //
        pub const m_strImmobilizeTargetSound: usize = 0x1940; //
        pub const m_strArmingSound: usize = 0x1950; //
        pub const m_GlitchModifier: usize = 0x1960; //
        pub const m_DebuffModifier: usize = 0x1970; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_FireBomb {
        pub const m_flDetonateTime: usize = 0xD18; //
        pub const m_flStartTime: usize = 0xD30; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Item_SmokeBomb_PreCast {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_MagicShock_ProcVData {
        pub const m_ProcParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_hDamageTrackModifier: usize = 0x748; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 18
    pub mod C_Sun {
        pub const m_fxSSSunFlareEffectIndex: usize = 0x858; //
        pub const m_fxSunFlareEffectIndex: usize = 0x85C; //
        pub const m_fdistNormalize: usize = 0x860; //
        pub const m_vSunPos: usize = 0x864; //
        pub const m_vDirection: usize = 0x870; //
        pub const m_iszEffectName: usize = 0x880; //
        pub const m_iszSSEffectName: usize = 0x888; //
        pub const m_clrOverlay: usize = 0x890; //
        pub const m_bOn: usize = 0x894; //
        pub const m_bmaxColor: usize = 0x895; //
        pub const m_flSize: usize = 0x898; //
        pub const m_flHazeScale: usize = 0x89C; //
        pub const m_flRotation: usize = 0x8A0; //
        pub const m_flHDRColorScale: usize = 0x8A4; //
        pub const m_flAlphaHaze: usize = 0x8A8; //
        pub const m_flAlphaScale: usize = 0x8AC; //
        pub const m_flAlphaHdr: usize = 0x8B0; //
        pub const m_flFarZScale: usize = 0x8B4; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 12
    pub mod CCitadel_Ability_Shiv_KillingBlowVData {
        pub const m_LeapModifier: usize = 0x1590; //
        pub const m_ActiveBuff: usize = 0x15A0; //
        pub const m_KillableModifier: usize = 0x15B0; //
        pub const m_AttackParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x16A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FlashParticle: usize = 0x1780; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_KillingBlowCastParticle: usize = 0x1860; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChargeUpSound: usize = 0x1940; //
        pub const m_OnKillSound: usize = 0x1950; //
        pub const m_flPreArrivalAttackStartTime: usize = 0x1960; //
        pub const m_flKillableGlowRange: usize = 0x1964; //
        pub const m_flGlowMinTime: usize = 0x1968; //
    }
    // Parent: CCitadelBaseShivAbility
    // Field count: 8
    pub mod CCitadel_Ability_ShivDash {
        pub const m_vStartPosition: usize = 0xCA0; //
        pub const m_vDashDirection: usize = 0xCAC; //
        pub const m_bIsDashing: usize = 0xCB8; //
        pub const m_bStartedInAir: usize = 0xCB9; //
        pub const m_vecHitEnemies: usize = 0xCC0; //
        pub const m_vecLastPosition: usize = 0xCD8; //
        pub const m_nReductionsLeft: usize = 0xCE4; //
        pub const m_flStuckTime: usize = 0xF18; //
    }
    // Parent: CCitadel_Ability_TrooperGrenade
    // Field count: 0
    pub mod CCitadel_Ability_TrooperNeutralGrenade {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PlayerPinged {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_TrackingProjectileApplyModifier {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Rutger_Pulse_Target {
        pub const m_vAuraCenter: usize = 0x1A0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityGenericPerson3VData {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityPsychicLiftVData {
        pub const m_LiftModifier: usize = 0x1590; //
        pub const m_TargetParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetCastSound: usize = 0x1680; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_ColossusActive_VData {
        pub const m_AuraModifier: usize = 0x638; //
        pub const m_ShieldParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Discord_Enemy {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_BulletArmorShredder_Proc {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_BonusDamagePercentVData {
        pub const m_bSelfish: usize = 0x638; //
    }
    // Parent: CEntityComponent
    // Field count: 1
    pub mod CHitboxComponent {
        pub const m_bvDisabledHitGroups: usize = 0x24; //
    }
    // Parent: C_BaseTrigger
    // Field count: 9
    pub mod C_ColorCorrectionVolume {
        pub const m_LastEnterWeight: usize = 0x998; //
        pub const m_LastEnterTime: usize = 0x99C; //
        pub const m_LastExitWeight: usize = 0x9A0; //
        pub const m_LastExitTime: usize = 0x9A4; //
        pub const m_bEnabled: usize = 0x9A8; //
        pub const m_MaxWeight: usize = 0x9AC; //
        pub const m_FadeDuration: usize = 0x9B0; //
        pub const m_Weight: usize = 0x9B4; //
        pub const m_lookupFilename: usize = 0x9B8; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_PrecipitationBlocker {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_Gun_Spikes {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 7
    pub mod CCitadel_Ability_IceBeam {
        pub const m_flNextDamageTick: usize = 0x1288; //
        pub const m_vStart: usize = 0x128C; //
        pub const m_vEnd: usize = 0x1298; //
        pub const m_vecEntitiesHit: usize = 0x12E0; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_vBeamAimPos: usize = 0x12F8; //
        pub const m_angBeamAngles: usize = 0x1308; //
        pub const m_bNeedsBeamReset: usize = 0x1320; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Sleep {
    }
    // Parent: CModifierVData
    // Field count: 27
    pub mod CCitadelModifierVData {
        pub const m_bIsBuildup: usize = 0x3F0; //
        pub const m_bNetworkValuesForStatsPreview: usize = 0x3F1; //
        pub const m_vecAutoRegisterModifierValueFromAbilityPropertyName: usize = 0x3F8; //
        pub const m_bCasterCountsAsAssister: usize = 0x410; //
        pub const m_flLingeringAssistWindow: usize = 0x414; //
        pub const m_bDurationCanBeTimeScaled: usize = 0x418; //
        pub const m_bDurationReducible: usize = 0x419; //
        pub const m_eTimeScaleSource: usize = 0x41C; //
        pub const m_bDurationAffectedByEffectiveness: usize = 0x420; //
        pub const m_vecSetAndTrackedAnimGraphParams: usize = 0x428; //
        pub const m_vecSetAndTrackedBodyGroups: usize = 0x440; //
        pub const m_eDrawOverheadStatus: usize = 0x458; //
        pub const m_bReverseHudProgressBar: usize = 0x45C; //
        pub const m_strSmallIconCssClass: usize = 0x460; //
        pub const m_strHintText: usize = 0x468; //
        pub const m_strHudIcon: usize = 0x470; //
        pub const m_eHudDisplayLocation: usize = 0x480; //
        pub const m_strHudMessageText: usize = 0x488; //
        pub const m_bIsHiddenOverhead: usize = 0x490; //
        pub const m_vecAlwaysShowInStatModifierUI: usize = 0x498; //
        pub const m_OnCreateResponse: usize = 0x4B0; //
        pub const m_cameraSequenceCreated: usize = 0x4E8; //
        pub const m_bEndCreatedSequenceOnRemove: usize = 0x570; //
        pub const m_cameraSequenceRemoved: usize = 0x578; //
        pub const m_sExpiredSound: usize = 0x600; //
        pub const m_FootstepOverride: usize = 0x610; //
        pub const m_FootstepAdditional: usize = 0x628; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 9
    pub mod CModifierLockDownDebuffVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AOEParticleCaster: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AOEParticleEnemy: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AOEParticleOthers: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strFollowLoop: usize = 0x9B8; //
        pub const m_strExplodeSound: usize = 0x9C8; //
        pub const m_strEscapedSound: usize = 0x9D8; //
        pub const m_RootModifier: usize = 0x9E8; //
        pub const m_BulletResistModifier: usize = 0x9F8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Bounce_Pad_Ally {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 2
    pub mod CModifierAirRaidVData {
        pub const m_SlowModifier: usize = 0x668; //
        pub const m_strWeaponShootSound: usize = 0x678; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_Bull_HealVData {
        pub const m_AuraModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Passive_Cloak {
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_Item_ComboBreakerVData {
        pub const m_ComboBreakerModifier: usize = 0x15D8; //
        pub const m_HealModifier: usize = 0x15E8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_HalloweenMask {
    }
    // Parent: CCitadelModifierVData
    // Field count: 6
    pub mod CCitadel_Modifier_SettingSunThinker_VData {
        pub const m_TargetParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LingerParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LayerParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExplodeSound: usize = 0x9B8; //
        pub const m_strTargetingCompletedSound: usize = 0x9C8; //
    }
    // Parent: CCitadel_Modifier_Base
    // Field count: 0
    pub mod CCitadel_Modifier_Haunt_Damage {
    }
    // Parent: CCitadelModifierVData
    // Field count: 12
    pub mod CCitadel_Modifier_PsychicDagger_MakeDaggers_VData {
        pub const m_flDesatAmount: usize = 0x638; //
        pub const m_DesatTint: usize = 0x63C; //
        pub const m_SatTint: usize = 0x640; //
        pub const m_Outline: usize = 0x644; //
        pub const m_DaggerShot: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerSpawn: usize = 0x728; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerAoE: usize = 0x808; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerTargetPreview: usize = 0x8E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerShotFail: usize = 0x9C8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerFireSound: usize = 0xAA8; //
        pub const m_DaggerMissSound: usize = 0xAB8; //
        pub const m_LastDaggerMissSound: usize = 0xAC8; //
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_GhostBloodShard {
        pub const m_flMinSlowAmount: usize = 0xC0; //
        pub const m_flMoveSpeedPenaltyPerStack: usize = 0xC4; //
        pub const m_flSlowDuration: usize = 0xC8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_LifestrikeGauntlets_VData {
        pub const m_SwingParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x748; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_AmmoScavenger {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Objective_RegenVData {
        pub const m_flOutOfCombatHealthRegen: usize = 0x638; //
        pub const m_flOutOfCombatRegenDelay: usize = 0x63C; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_ApplyDebuff_Proc {
    }
    // Parent: CBaseAnimGraph
    // Field count: 0
    pub mod C_Citadel_PickupItemSpawner {
    }
    // Parent: CBaseAnimGraph
    // Field count: 0
    pub mod CCitadel_HeroTestOrbSpawner {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_Item_RescueBeam {
        pub const m_bCanPull: usize = 0xCB8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityCadenceLullabyVData {
        pub const m_SleepAOEModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Chrono_TimeWall_Effect {
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_WeaponUpgrade_SurgingPowerVData {
        pub const m_ModifierSurgingPower: usize = 0x15D8; //
        pub const m_CastTargetEffect: usize = 0x15E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_FireRateAuraVData {
        pub const m_FireRateAuraSourceParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_Intrinsic_Base
    // Field count: 0
    pub mod CCitadel_Modifier_MagicClarityWatcher {
    }
    // Parent: CCitadelModelEntity
    // Field count: 3
    pub mod C_Citadel_Shield {
        pub const m_bAllowRotatingUp: usize = 0x860; //
        pub const m_bFixedPosition: usize = 0x861; //
        pub const m_flShieldOffset: usize = 0x864; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Metal {
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityGangActivityVData {
        pub const m_AbilitySwap: usize = 0x1590; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CCitadel_Ability_SettingSun_VData {
        pub const m_BeamTargetParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_UnitTargetParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SettingSunThinkerModifier: usize = 0x1750; //
        pub const m_flSSCameraPreviewOffset: usize = 0x1760; //
        pub const m_flSSCameraPreviewSpeed: usize = 0x1764; //
        pub const m_flSSCameraPreviewDistance: usize = 0x1768; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Vandal_Ability03 {
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_Burrow_VData {
        pub const m_BurrowPlayerParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flDesatAmount: usize = 0x718; //
        pub const m_DesatTint: usize = 0x71C; //
        pub const m_SatTint: usize = 0x720; //
        pub const m_Outline: usize = 0x724; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CModifier_BansheeSlugs_Headshot {
        pub const m_nDebuffsTotal: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ReturnFire {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_BulletResistReductionStack {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 2
    pub mod CFuncFoliageVData {
        pub const m_BulletImpactParticle: usize = 0x28; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BulletExitParticle: usize = 0x108; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Arcane_Eater_Debuff {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 2
    pub mod CCitadel_Modifier_BaseBulletPreRollProc {
        pub const m_nSuppressProcShotID: usize = 0x168; //
        pub const m_vecProcdBulletIDs: usize = 0x170; //
    }
    // Parent: CPlayer_MovementServices_Humanoid
    // Field count: 7
    pub mod CCitadelPlayer_MovementServices {
        pub const m_vPositionDeltaVelocity: usize = 0x218; //
        pub const m_bToggleDuckActive: usize = 0x248; //
        pub const m_vecPogoVelocity: usize = 0x24C; //
        pub const m_vecSupport: usize = 0x258; //
        pub const m_bColliding: usize = 0x264; //
        pub const m_bLandedOnGround: usize = 0x265; //
        pub const m_bHasFreeCursor: usize = 0x266; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_Gunslinger_DemonMarkVData {
        pub const m_ProcEffect: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffModifier: usize = 0x718; //
        pub const m_SlowModifier: usize = 0x728; //
        pub const m_CasterMarkTriggerSound: usize = 0x738; //
        pub const m_VictimMarkTriggerSound: usize = 0x748; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CAbilityWreckingBallVData {
        pub const m_SummonParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SummonReadyParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SummonParticleAttachment: usize = 0x1750; //
        pub const m_ExplodeParticle: usize = 0x1758; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AutoThrowModifier: usize = 0x1838; //
        pub const m_HoldingBallLoop: usize = 0x1848; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_Gravity_Lasso_Self {
        pub const m_bHasUsedBouncePad: usize = 0xC0; //
        pub const m_vCastTargets: usize = 0xC8; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_PsychicLift {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Burning {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_InFountain {
    }
    // Parent: None
    // Field count: 0
    pub mod CSharedGapTypeQueryRegistration {
    }
    // Parent: C_BarnLight
    // Field count: 3
    pub mod C_OmniLight {
        pub const m_flInnerAngle: usize = 0xBA0; //
        pub const m_flOuterAngle: usize = 0xBA4; //
        pub const m_bShowLight: usize = 0xBA8; //
    }
    // Parent: CCitadel_Modifier_ScalingPowerUp
    // Field count: 0
    pub mod CCitadel_Modifier_PowerUp_Survival {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_Crescendo_PostAOE {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ChargedTackleActive {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_MobileResupply {
    }
    // Parent: CCitadel_Modifier_Base
    // Field count: 0
    pub mod CCitadel_Modifier_MagicCarpet_Summon {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_RescueBeamVData {
        pub const m_BeamParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod C_NPC_HornetDrone {
    }
    // Parent: C_CitadelProjectile
    // Field count: 1
    pub mod CCitadel_Projectile_MagicBolt {
        pub const bIsCloneProjectile: usize = 0x8E0; //
    }
    // Parent: CCitadelModifier
    // Field count: 6
    pub mod CModifier_Mirage_SandPhantom {
        pub const m_particleStart: usize = 0xC0; //
        pub const m_particleEnd: usize = 0xC4; //
        pub const m_particleTrail: usize = 0xC8; //
        pub const m_vecStartPosition: usize = 0xCC; //
        pub const m_flStartDelay: usize = 0xD8; //
        pub const m_vecApplyOffset: usize = 0xDC; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CModifier_Synth_Grasp_Victim {
        pub const m_vecOrigin: usize = 0xC0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Cadence_Lullaby {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_GangActivity {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_ShieldedSentry_VData {
        pub const m_InnateModifier: usize = 0x1590; //
        pub const m_DebuffModifier: usize = 0x15A0; //
        pub const m_flDamageFalloffEndScale: usize = 0x15B0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Hornet_Snipe {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadelBaseAbilityServerOnly {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 3
    pub mod CScaleFunctionVData {
        pub const m_eSpecificStatScaleType: usize = 0x28; //
        pub const m_flStatScale: usize = 0x2C; //
        pub const m_bFunctionDisabled: usize = 0x30; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_Thumper_2_AuraVData {
        pub const m_AoEParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 11
    pub mod CCitadel_Modifier_ItemWalkBackVData {
        pub const m_IdleParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RunningParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flStopDistance: usize = 0x7F8; //
        pub const m_flMaxSpeedDistance: usize = 0x7FC; //
        pub const m_flSlowSpeed: usize = 0x800; //
        pub const m_flFastSpeed: usize = 0x804; //
        pub const m_flVerticalOffset: usize = 0x808; //
        pub const m_flTolerance: usize = 0x80C; //
        pub const m_flRepathTime: usize = 0x810; //
        pub const m_flAutoRunTime: usize = 0x814; //
        pub const m_flTimeToStartAutoRun: usize = 0x818; //
    }
    // Parent: C_PointEntity
    // Field count: 0
    pub mod CCitadelItemPickupRejuvHeroTestInfoSpawn {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CAbility_Synth_PlasmaFlux {
        pub const m_bTeleported: usize = 0xCB0; //
        pub const m_flProjectileLaunchTime: usize = 0xCB4; //
        pub const m_flProjectileExpireTime: usize = 0xCB8; //
        pub const m_hActiveProjectile: usize = 0xCBC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_Gunslinger_DemonCarbineVData {
        pub const m_TracerParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FullyChargedParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strFullyCharged: usize = 0x7F8; //
        pub const m_strShotSound: usize = 0x808; //
    }
    // Parent: CCitadel_Ability_PrimaryWeaponVData
    // Field count: 1
    pub mod CCitadel_Ability_ShivWeapon_VData {
        pub const m_flPushForce: usize = 0x15E0; //
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_Nano_PredatoryStatue {
        pub const m_GameTimeEnabled: usize = 0x100; //
        pub const m_LastCatInAreaTime: usize = 0x104; //
        pub const m_bIsAttacking: usize = 0x108; //
        pub const m_iTargetID: usize = 0x10C; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Guiding_ArrowVData {
        pub const m_GlowEnemeyModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_DeathTax {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Low_Health_GlowVData {
        pub const m_GlowParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ParriedStun {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_HunterAuraTarget {
        pub const m_flDebuffScale: usize = 0x168; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_CQC_Proc {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_SlowImmunityVData {
        pub const m_ImmunityModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Discord_Friendly {
        pub const m_flHealPerSecond: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifierObscuredVData {
        pub const m_flHideDuration: usize = 0x638; //
        pub const m_flRevealDuration: usize = 0x63C; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_HeldItemPickupAuraVData {
        pub const m_strFilterAbilityName: usize = 0x680; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_DamageRecycler {
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod CFilterClass {
        pub const m_iFilterClass: usize = 0x5B8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CCitadel_Ability_Magician_BigBoltVData {
        pub const m_ChargeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShootDelayParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CasterModifier: usize = 0x1750; //
        pub const m_BoltHitModifier: usize = 0x1760; //
        pub const m_strBoltDelay: usize = 0x1770; //
        pub const m_strBoltFire: usize = 0x1780; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityTargetdummy2VData {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Wrecker_Ultimate {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_BulletArmorShredder_ProcVData {
        pub const m_DebuffModifier: usize = 0x668; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_Obscured {
        pub const m_flStartObscuredAmount: usize = 0xC0; //
        pub const m_AmbientParticles: usize = 0xC8; // CUtlVectorFixedGrowable<ParticleIndex_t,3>
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_ItemPickupAura {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Thumper_PullAOE {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_Camouflage {
    }
    // Parent: CAI_NPC_TrooperVData
    // Field count: 7
    pub mod CNPC_TrooperBossVData {
        pub const m_bMitigateDamageFromPlayers: usize = 0x17C8; //
        pub const m_flPlayerAutoAttackRange: usize = 0x17CC; //
        pub const m_flMinMeleeAttackTime: usize = 0x17D0; //
        pub const m_flInvulRange: usize = 0x17D4; //
        pub const m_sAngryStart: usize = 0x17D8; //
        pub const m_sAngryLoop: usize = 0x17E8; //
        pub const m_sAngryStop: usize = 0x17F8; //
    }
    // Parent: CBaseFilter
    // Field count: 0
    pub mod CFilterLOS {
    }
    // Parent: None
    // Field count: 2
    pub mod C_EconItemAttribute {
        pub const m_iAttributeDefinitionIndex: usize = 0x30; //
        pub const m_flValue: usize = 0x34; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityTokamakRadianceVData {
        pub const m_RadianceModifier: usize = 0x1590; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_Gunslinger_DemonCarbineVData {
        pub const m_flShotTimeScaleLingerDuration: usize = 0x1590; //
        pub const m_ChargingModifier: usize = 0x1598; //
        pub const m_DebuffModifier: usize = 0x15A8; //
        pub const m_cameraDemonCarbineShotFired: usize = 0x15B8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_ViperVenomProcWatcher {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_Perched_Predator {
        pub const m_hActiveProjectile: usize = 0xCA0; //
    }
    // Parent: None
    // Field count: 5
    pub mod ice_path_shard_model_desc_t {
        pub const m_nModelID: usize = 0x8; //
        pub const m_vecPanelSize: usize = 0xC; //
        pub const m_vecPanelVertices: usize = 0x18; //
        pub const m_flThickness: usize = 0x30; //
        pub const m_SurfacePropStringToken: usize = 0x34; //
    }
    // Parent: None
    // Field count: 3
    pub mod EngineCountdownTimer {
        pub const m_duration: usize = 0x8; //
        pub const m_timestamp: usize = 0xC; //
        pub const m_timescale: usize = 0x10; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Projectile_Synth_Barrage {
    }
    // Parent: CCitadelModifierAura_Cone
    // Field count: 2
    pub mod CCitadel_Modifier_Bull_Heal_Aura {
        pub const m_playerAngles: usize = 0xF0; //
        pub const m_ConeParticle: usize = 0xFC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_ShadowCloneVData {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Nano_Shadow_Debuff {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_WreckerScrapBlast {
    }
    // Parent: CitadelAbilityVData
    // Field count: 16
    pub mod CAbilityMeleeParryVData {
        pub const m_flWhiffDuration: usize = 0x1590; //
        pub const m_flMovementRestrictionTime: usize = 0x1594; //
        pub const m_flActiveTime: usize = 0x1598; //
        pub const m_flParryEndVisualTime: usize = 0x159C; //
        pub const m_flSuccessActiveTime: usize = 0x15A0; //
        pub const m_flBossVictimNoMeleeTime: usize = 0x15A4; //
        pub const m_flBossVictimCalmTime: usize = 0x15A8; //
        pub const m_SuccessfulParryParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ActiveParryParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strSuccessfulParrySound: usize = 0x1770; //
        pub const m_ParryActiveModifier: usize = 0x1780; //
        pub const m_ParryVictimModifier: usize = 0x1790; //
        pub const m_ParryCooldownModifier: usize = 0x17A0; //
        pub const m_ParryEndVisualModifier: usize = 0x17B0; //
        pub const m_ParryBossVictimNoMeleeModifier: usize = 0x17C0; //
        pub const m_ParryBossVictimCalmModifier: usize = 0x17D0; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 1
    pub mod C_Citadel_BreakableProp {
        pub const m_nHitIndex: usize = 0xAF8; //
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProc
    // Field count: 1
    pub mod CCitadel_Modifier_EmpowerBullet {
        pub const m_BuffedShotId: usize = 0x1F8; //
    }
    // Parent: CCitadelBaseYamatoAbility
    // Field count: 22
    pub mod CCitadel_Ability_FlyingStrike {
        pub const m_desatVolIdx: usize = 0xCB8; //
        pub const m_bShadowFormCast: usize = 0xCBC; //
        pub const m_vYamatoCastPos: usize = 0xCC0; //
        pub const m_vTargetCastPos: usize = 0xCCC; //
        pub const m_flFlyingToTargetStartTime: usize = 0xCD8; //
        pub const m_flEndAttackTime: usize = 0xCDC; //
        pub const m_flGrappleStartTime: usize = 0xCE0; //
        pub const m_flGrappleArriveTime: usize = 0xCE4; //
        pub const m_flAttackLatchTime: usize = 0xCE8; //
        pub const m_vAttackLatchPos: usize = 0xCEC; //
        pub const m_hTarget: usize = 0xCF8; //
        pub const m_bIsTargetAlly: usize = 0xCFC; //
        pub const m_flGrappleShotAttackTime: usize = 0xD00; //
        pub const m_rgPath: usize = 0xD08; //
        pub const m_nPathIdx: usize = 0xDF8; //
        pub const m_nPathSize: usize = 0xDFC; //
        pub const m_flPathLength: usize = 0xE00; //
        pub const m_vFlyingInitialOffsetToPath: usize = 0xE04; //
        pub const flDistFlown: usize = 0xE10; //
        pub const m_vLastSafePos: usize = 0xE14; //
        pub const m_nGrappleTravelEffect: usize = 0xF00; //
        pub const m_bPathDirty: usize = 0xF58; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_ShivDash {
        pub const m_bUseTrail: usize = 0xF8; //
        pub const m_bUseEchoEffect: usize = 0xF9; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_TargetPracticeSelfVData {
        pub const m_TracerParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strWeaponShootSound: usize = 0x718; //
        pub const m_strBulletWhizSound: usize = 0x728; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_Cadence_SleepAOEVData {
        pub const m_AuraParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_Magician_CopyUlt {
        pub const m_bHasUsedCopiedUlt: usize = 0xD80; //
        pub const m_bHasCopiedUlt: usize = 0xD81; //
        pub const m_bIsModelSwapped: usize = 0xD82; //
        pub const m_pCopyUltimateAbility: usize = 0xD84; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_Crescendo_InAOE {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Viper_SlideBuff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Astro_ShotgunBuffVData {
        pub const m_DebuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Bull_Heal_TargetVData {
        pub const m_DrainParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_Intrinsic_BaseVData
    // Field count: 2
    pub mod CCitadel_Modifier_NapalmProjectileVData {
        pub const m_DebuffModifier: usize = 0x638; //
        pub const m_SlowModifier: usize = 0x648; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_WeaponEaterVData {
        pub const m_WeaponEaterTracker: usize = 0x15D8; //
    }
    // Parent: CCitadel_Modifier_ChainLightningEffect
    // Field count: 0
    pub mod CCitadel_Modifier_Galvanic_Storm_Effect {
    }
    // Parent: CitadelItemVData
    // Field count: 3
    pub mod CItemAOERootVData {
        pub const m_AOEParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strRootTargetSound: usize = 0x16B8; //
        pub const m_TetherModifier: usize = 0x16C8; //
    }
    // Parent: CitadelItemVData
    // Field count: 3
    pub mod CCitadel_Item_TrackingProjectileApplyModifierVData {
        pub const m_ProjectileImpactParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetModifier: usize = 0x16B8; //
        pub const m_FriendlyOnlyModifier: usize = 0x16C8; //
    }
    // Parent: C_CitadelTrackedProjectile
    // Field count: 1
    pub mod CCitadel_Projectile_HookBlade {
        pub const bIsReturning: usize = 0x8E0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Rutger_CheatDeath_Activated_VData {
        pub const m_ActivatedParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadelBaseShivAbility {
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_ReturnFireVData {
        pub const m_AttackerHitFx: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SpiritReflectTracerReplacement: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAttackerHitSound: usize = 0x8D8; //
        pub const m_strHitProcSound: usize = 0x8E8; //
    }
    // Parent: None
    // Field count: 5
    pub mod C_BaseFlex__Emphasized_Phoneme {
        pub const m_sClassName: usize = 0x0; //
        pub const m_flAmount: usize = 0x18; //
        pub const m_bRequired: usize = 0x1C; //
        pub const m_bBasechecked: usize = 0x1D; //
        pub const m_bValid: usize = 0x1E; //
    }
    // Parent: None
    // Field count: 2
    pub mod IntervalTimer {
        pub const m_timestamp: usize = 0x8; //
        pub const m_nWorldGroupId: usize = 0xC; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Citadel_Projectile_SettingSun {
    }
    // Parent: CCitadel_Modifier_SilencedVData
    // Field count: 3
    pub mod CCitadel_Modifier_BubbleVData {
        pub const m_ExplodeParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x9B8; //
        pub const m_BuffModifier: usize = 0x9C8; //
    }
    // Parent: CNPC_SimpleAnimatingAIVData
    // Field count: 12
    pub mod CNPC_FieldSentryVData {
        pub const m_LaserSightParticle: usize = 0x108; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_KillExplosionParticle: usize = 0x1E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DeployProgressModifier: usize = 0x2C8; //
        pub const m_sSpawnSound: usize = 0x2D8; //
        pub const m_sKillExplosionSound: usize = 0x2E8; //
        pub const m_sTargetAcquiredLocalSound: usize = 0x2F8; //
        pub const m_sTargetAcquiredSound: usize = 0x308; //
        pub const m_flIdleTurnSpeed: usize = 0x318; //
        pub const m_flIdleTurnAngles: usize = 0x31C; //
        pub const m_flTrooperTakeDamageMult: usize = 0x320; //
        pub const m_flNeutralTakeDamageMulti: usize = 0x324; //
        pub const m_flNotifyEventTime: usize = 0x328; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_HookSelf {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_VoidSphereVData {
        pub const m_BubbleModifier: usize = 0x1590; //
        pub const m_strCastEffect: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAllyPositionPreview: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ExplosiveShots {
    }
    // Parent: C_BaseTrigger
    // Field count: 3
    pub mod CCitadelCatapultTrigger {
        pub const m_vLaunchTarget: usize = 0x998; //
        pub const m_flLaunchSpeed: usize = 0x9A4; //
        pub const m_nameTarget: usize = 0x9A8; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Upgrade_AerialAssault {
    }
    // Parent: C_BaseModelEntity
    // Field count: 9
    pub mod C_FuncLadder {
        pub const m_vecLadderDir: usize = 0x858; //
        pub const m_Dismounts: usize = 0x868; // CUtlVector<CHandle<C_InfoLadderDismount>>
        pub const m_vecLocalTop: usize = 0x880; //
        pub const m_vecPlayerMountPositionTop: usize = 0x88C; //
        pub const m_vecPlayerMountPositionBottom: usize = 0x898; //
        pub const m_flAutoRideSpeed: usize = 0x8A4; //
        pub const m_bDisabled: usize = 0x8A8; //
        pub const m_bFakeLadder: usize = 0x8A9; //
        pub const m_bHasSlack: usize = 0x8AA; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_Synth_Affliction_Debuff_VData {
        pub const m_EffectParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Synth_PlasmaFlux_WeaponDamage {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Cadence_SilenceContraptionsVData {
        pub const m_DebuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_TangoTether_Tether {
        pub const m_fHealingSoundBuildup: usize = 0xF8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_MeleeCharge {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Fervor_Bonuses_VData {
        pub const m_BonusesParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ActivateBonusesSound: usize = 0x718; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_SilencerProcActiveVData {
        pub const m_TracerParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SilencerActiveParticle: usize = 0x748; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SilenceActiveModifier: usize = 0x828; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_HalloweenMaskVData {
        pub const m_nNumMasks: usize = 0x638; //
        pub const m_HalloweenMask: usize = 0x640; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_SoundOpvarSetAABBEntity
    // Field count: 0
    pub mod C_SoundOpvarSetOBBEntity {
    }
    // Parent: CPlayerPawnComponent
    // Field count: 4
    pub mod CPlayer_WeaponServices {
        pub const m_hMyWeapons: usize = 0x40; //
        pub const m_hActiveWeapon: usize = 0x58; //
        pub const m_hLastWeapon: usize = 0x5C; //
        pub const m_iAmmo: usize = 0x60; //
    }
    // Parent: CUnitStatusOverlay
    // Field count: 1
    pub mod CUnitStatusOverlayOld {
        pub const m_flUIScale: usize = 0xB00; //
    }
    // Parent: None
    // Field count: 3
    pub mod CAttributeManager__cached_attribute_float_t {
        pub const flIn: usize = 0x0; //
        pub const iAttribHook: usize = 0x8; //
        pub const flOut: usize = 0x10; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Trapper_SpiderShield {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Bolo {
        pub const m_hRingEffect: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadelModifierDustStormAuraApplyVData {
        pub const m_DebuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_BulletFlurryWindup {
    }
    // Parent: CBaseAnimGraph
    // Field count: 0
    pub mod CCitadelAnimatingModelEntity {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Operative_Blindside_VData {
        pub const m_EnemyDebuffModifier: usize = 0x1590; //
        pub const m_ExplodeParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionSound: usize = 0x1680; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TargetPracticeDebuff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Chrono_PulseGrenade_PulseArea {
    }
    // Parent: CCitadel_Modifier_BaseShield
    // Field count: 0
    pub mod CCitadel_Modifier_Wraith_ProjectMind_Shield {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_FlameDashBurn {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_ZipLineBoost_VData {
        pub const m_ZipboostModifier: usize = 0x1590; //
        pub const m_flTimeToActivate: usize = 0x15A0; //
        pub const m_flTimeForHint: usize = 0x15A4; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 10
    pub mod CCitadel_Ability_SuperNeutralCharge {
        pub const m_bPreparing: usize = 0xE60; //
        pub const m_bTackling: usize = 0xE61; //
        pub const m_flTackleStartTime: usize = 0xE64; //
        pub const m_flTackleDuration: usize = 0xE68; //
        pub const m_vecTackleDir: usize = 0xE6C; //
        pub const m_vecLastPosition: usize = 0xE78; //
        pub const m_nStuckFramesCount: usize = 0xE84; //
        pub const m_vecHitEnemies: usize = 0xE88; //
        pub const m_flPrepareStartTime: usize = 0xEA0; //
        pub const m_nDistancePreview: usize = 0xEA4; //
    }
    // Parent: None
    // Field count: 3
    pub mod STrooperFOWEntity {
        pub const m_nEntIndex: usize = 0x30; //
        pub const m_nTeam: usize = 0x34; //
        pub const m_nPositionXY: usize = 0x36; //
    }
    // Parent: None
    // Field count: 8
    pub mod VPhysicsCollisionAttribute_t {
        pub const m_nInteractsAs: usize = 0x8; //
        pub const m_nInteractsWith: usize = 0x10; //
        pub const m_nInteractsExclude: usize = 0x18; //
        pub const m_nEntityId: usize = 0x20; //
        pub const m_nOwnerId: usize = 0x24; //
        pub const m_nHierarchyId: usize = 0x28; //
        pub const m_nCollisionGroup: usize = 0x2A; //
        pub const m_nCollisionFunctionMask: usize = 0x2B; //
    }
    // Parent: None
    // Field count: 0
    pub mod CLogicRelayAPI {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_HeldItemPickupAura {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod CCitadel_Projectile_Petrify {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_Item_Bubble {
        pub const m_flEndTime: usize = 0xCB8; //
    }
    // Parent: C_BaseEntity
    // Field count: 37
    pub mod C_EnvVolumetricFogController {
        pub const m_flScattering: usize = 0x560; //
        pub const m_TintColor: usize = 0x564; //
        pub const m_flAnisotropy: usize = 0x568; //
        pub const m_flFadeSpeed: usize = 0x56C; //
        pub const m_flDrawDistance: usize = 0x570; //
        pub const m_flFadeInStart: usize = 0x574; //
        pub const m_flFadeInEnd: usize = 0x578; //
        pub const m_flIndirectStrength: usize = 0x57C; //
        pub const m_nVolumeDepth: usize = 0x580; //
        pub const m_fFirstVolumeSliceThickness: usize = 0x584; //
        pub const m_nIndirectTextureDimX: usize = 0x588; //
        pub const m_nIndirectTextureDimY: usize = 0x58C; //
        pub const m_nIndirectTextureDimZ: usize = 0x590; //
        pub const m_vBoxMins: usize = 0x594; //
        pub const m_vBoxMaxs: usize = 0x5A0; //
        pub const m_bActive: usize = 0x5AC; //
        pub const m_flStartAnisoTime: usize = 0x5B0; //
        pub const m_flStartScatterTime: usize = 0x5B4; //
        pub const m_flStartDrawDistanceTime: usize = 0x5B8; //
        pub const m_flStartAnisotropy: usize = 0x5BC; //
        pub const m_flStartScattering: usize = 0x5C0; //
        pub const m_flStartDrawDistance: usize = 0x5C4; //
        pub const m_flDefaultAnisotropy: usize = 0x5C8; //
        pub const m_flDefaultScattering: usize = 0x5CC; //
        pub const m_flDefaultDrawDistance: usize = 0x5D0; //
        pub const m_bStartDisabled: usize = 0x5D4; //
        pub const m_bEnableIndirect: usize = 0x5D5; //
        pub const m_bIndirectUseLPVs: usize = 0x5D6; //
        pub const m_bIsMaster: usize = 0x5D7; //
        pub const m_hFogIndirectTexture: usize = 0x5D8; //
        pub const m_nForceRefreshCount: usize = 0x5E0; //
        pub const m_fNoiseSpeed: usize = 0x5E4; //
        pub const m_fNoiseStrength: usize = 0x5E8; //
        pub const m_vNoiseScale: usize = 0x5EC; //
        pub const m_fWindSpeed: usize = 0x5F8; //
        pub const m_vWindDirection: usize = 0x5FC; //
        pub const m_bFirstTime: usize = 0x608; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ShieldGuy_Ability01 {
    }
    // Parent: CitadelAbilityVData
    // Field count: 14
    pub mod CCitadel_Ability_Nano_Pounce_InstantVData {
        pub const m_LeapModifier: usize = 0x1590; //
        pub const m_ActiveBuff: usize = 0x15A0; //
        pub const m_SlowModifier: usize = 0x15B0; //
        pub const m_AttackParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FlashParticle: usize = 0x16A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1780; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSlowParticle: usize = 0x1860; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PrimaryHitParticle: usize = 0x1940; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AttackSound: usize = 0x1A20; //
        pub const m_strExplodeSound: usize = 0x1A30; //
        pub const m_flAttackTimePhase01: usize = 0x1A40; //
        pub const m_flAttackTimePhase02: usize = 0x1A44; //
        pub const m_flAllyMinTargetRange: usize = 0x1A48; //
        pub const m_flTargetVerticalOffset: usize = 0x1A4C; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_WreckingBall {
        pub const m_bHoldingBall: usize = 0xCD8; //
    }
    // Parent: CCitadel_Ability_PrimaryWeapon
    // Field count: 0
    pub mod CCitadel_Ability_PrimaryWeapon_BeamWeapon {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_ArcaneEaterProcVData {
        pub const m_StealWatcherModifier: usize = 0x668; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SuperNeutralChargePrepare {
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 7
    pub mod CCitadel_Modifier_BaseBulletPreRollProcVData {
        pub const m_bRollOnceForAllBulletsInAShot: usize = 0x668; //
        pub const m_flMaxBulletsToProcInShot: usize = 0x66C; //
        pub const m_bCanProcMultipleTimesFromSameShot: usize = 0x670; //
        pub const m_bRequiresTargetFilter: usize = 0x671; //
        pub const m_bCanBeEvaded: usize = 0x672; //
        pub const m_TracerAdditionParticle: usize = 0x678; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_OnBulletRolledProcSound: usize = 0x758; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Citadel_Projectile_WreckingBall {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierTangoTetherTargetVData {
        pub const m_GrappleRopeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_Vandal_PillarVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PetrifyModifier: usize = 0x1670; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_Tokamak_AllySmokeAOE_VData {
        pub const m_AuraParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_ActiveReload {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifierTier3BossInvulnVData {
        pub const m_ShieldParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flShieldRadius: usize = 0x718; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Fathom_ScaldingSpray_Target_VData {
        pub const m_DrainParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CAbility_Mirage_Teleport_VData {
        pub const m_BuffModifier: usize = 0x1590; //
        pub const m_ImmunityModifier: usize = 0x15A0; //
        pub const m_FireRateModifier: usize = 0x15B0; //
        pub const m_TeleportStartParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportEndParticle: usize = 0x16A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strArriveSound: usize = 0x1780; //
        pub const m_strDepartSound: usize = 0x1790; //
        pub const m_strChannelDestinationSound: usize = 0x17A0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CAbility_Rutger_RocketLauncher {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ChargedTacklePrepare {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_QuickSilverBuffVData {
        pub const m_RapidFireParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityPropertyBase {
    }
    // Parent: CPlayer_ObserverServices
    // Field count: 10
    pub mod CCitadelPlayer_ObserverServices {
        pub const m_nLastLocalPlayerObservedTeam: usize = 0x58; //
        pub const m_nLastObservedTeam: usize = 0x5C; //
        pub const m_nCurrentObservedTeam: usize = 0x60; //
        pub const m_hLastObserverTarget: usize = 0x64; //
        pub const m_hPreviousTeamTarget: usize = 0x68; //
        pub const m_hOverrideObserverTarget: usize = 0x6C; //
        pub const m_iOverrideObserverMode: usize = 0x70; //
        pub const m_iSecondsAfterDeathToAllowObserving: usize = 0x74; //
        pub const m_angTargetCamera: usize = 0x78; //
        pub const m_vTargetCameraPos: usize = 0x90; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 13
    pub mod C_PointCommentaryNode {
        pub const m_bActive: usize = 0xB00; //
        pub const m_bWasActive: usize = 0xB01; //
        pub const m_flEndTime: usize = 0xB04; //
        pub const m_flStartTime: usize = 0xB08; //
        pub const m_flStartTimeInCommentary: usize = 0xB0C; //
        pub const m_iszCommentaryFile: usize = 0xB10; //
        pub const m_iszTitle: usize = 0xB18; //
        pub const m_iszSpeakers: usize = 0xB20; //
        pub const m_iNodeNumber: usize = 0xB28; //
        pub const m_iNodeNumberMax: usize = 0xB2C; //
        pub const m_bListenedTo: usize = 0xB30; //
        pub const m_hViewPosition: usize = 0xB40; //
        pub const m_bRestartAfterRestore: usize = 0xB44; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_AnthemBuff {
    }
    // Parent: CCitadelModifier
    // Field count: 8
    pub mod CCitadel_Modifier_StormCloud {
        pub const m_flNextRandomLightningStrike: usize = 0xC0; //
        pub const m_flStartTime: usize = 0xC4; //
        pub const m_flRadiusIncrementPerSecond: usize = 0xC8; //
        pub const m_vCastPosition: usize = 0xCC; //
        pub const m_bFiredEndingSoonSound: usize = 0xD8; //
        pub const m_nLastTickForLightningCenterCalc: usize = 0xDC; //
        pub const m_vecLightningCenter: usize = 0xE0; //
        pub const m_nSatVolumeIndex: usize = 0xEC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_LightningBallVData {
        pub const m_ZapParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetScreenParticleEffect: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_Burning
    // Field count: 0
    pub mod CCitadel_Modifier_Afterburn_DOT {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_IncendiaryProjectile {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Disarmed {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Upgrade_SpellslingerHeadshots_Debuff {
        pub const m_tLastHeadshot: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_ScalingPowerUpVData {
        pub const m_vecModifierValues: usize = 0x638; // CUtlVector<ScalingPowerupDefinition_t>
        pub const m_flTimeMin: usize = 0x650; //
        pub const m_flTimeMax: usize = 0x654; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Thumper_2 {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_GangActivity_AbilitySwap {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_UltCombo_Self {
        pub const m_angles: usize = 0xC0; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CModifier_Upgrade_ArcaneMedallion_VData {
        pub const m_TriggeredModifier: usize = 0x668; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 12
    pub mod CCitadel_Ability_Dash {
        pub const m_flDashAngle: usize = 0xCA0; //
        pub const m_GroundDashExecuteTime: usize = 0xCA4; //
        pub const m_nLastGroundDashTick: usize = 0xCA8; //
        pub const m_flGroundDashCastTime: usize = 0xCAC; //
        pub const m_bTagCanActivateGroundDash: usize = 0xCB0; //
        pub const m_flGroundDashEndTime: usize = 0xCB8; //
        pub const m_flAirDashCastTime: usize = 0xCD0; //
        pub const m_flAirDashDragStartTime: usize = 0xCD4; //
        pub const m_nConsecutiveAirDashes: usize = 0xCD8; //
        pub const m_nConsecutiveDownDashes: usize = 0xCD9; //
        pub const m_bDownAirDash: usize = 0xCDA; //
        pub const m_hJumpAbility: usize = 0xE90; // CHandle<CCitadel_Ability_Jump>
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 27
    pub mod CAI_BaseNPCVData {
        pub const m_sModelName: usize = 0x28; //
        pub const m_hFootstepSounds: usize = 0x108; //
        pub const m_vecNavLinkMovementNames: usize = 0x110; //
        pub const m_nMaxHealth: usize = 0x128; //
        pub const m_vecIntrinsicModifiers: usize = 0x130; //
        pub const m_statusEffectMap: usize = 0x148; //
        pub const m_vecAttachments: usize = 0x150; //
        pub const m_flHeadDamageMultiplier: usize = 0x168; //
        pub const m_flChestDamageMultiplier: usize = 0x178; //
        pub const m_flStomachDamageMultiplier: usize = 0x188; //
        pub const m_flArmDamageMultiplier: usize = 0x198; //
        pub const m_flLegDamageMultiplier: usize = 0x1A8; //
        pub const m_nMaxAdditionalAmmoBalancingShots: usize = 0x1B8; //
        pub const m_bTakesDamage: usize = 0x1C8; //
        pub const m_strDamagedEffect: usize = 0x1D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_nRagdollHealth: usize = 0x2B0; //
        pub const m_DestructiblePartsOverrideByPartName: usize = 0x2B8; // CUtlOrderedMap<CUtlString,CDestructiblePartsSystemData_PartRuntimeData>
        pub const m_bAllowNonZUpMovement: usize = 0x2E0; //
        pub const m_bUseDynamicCollisionHull: usize = 0x2E1; //
        pub const m_bRequestCapsuleCollision: usize = 0x2E2; //
        pub const m_flCapsuleRadiusOverride: usize = 0x2E4; //
        pub const m_flCapsuleHeightOverride: usize = 0x2E8; //
        pub const m_vecActionDesiredShared: usize = 0x2F0; //
        pub const m_sPlayerKilledNpcSound: usize = 0x308; //
        pub const m_sCustomDeathHandshake: usize = 0x318; //
        pub const m_bEnableCodeDrivenAnimgraphMovement: usize = 0x320; //
        pub const m_defaultAdditionalMovementSettings: usize = 0x328; //
    }
    // Parent: None
    // Field count: 5
    pub mod audioparams_t {
        pub const localSound: usize = 0x8; //
        pub const soundscapeIndex: usize = 0x68; //
        pub const localBits: usize = 0x6C; //
        pub const soundscapeEntityListIndex: usize = 0x70; //
        pub const soundEventHash: usize = 0x74; //
    }
    // Parent: CCitadel_Item
    // Field count: 2
    pub mod CCitadel_Item_BaseProjectileAOEModifier {
        pub const m_vLaunchPosition: usize = 0xCB8; //
        pub const m_qLaunchAngle: usize = 0xCC4; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Trapper_Ability03 {
    }
    // Parent: CCitadelBaseYamatoAbility
    // Field count: 0
    pub mod CCitadel_Ability_HealingSlash {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_DebugIsVisibleToEnemyTeam {
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod C_NPC_MortarSentry {
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod C_NPC_FlyingDrone {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_Item_DivinersKevlar {
        pub const m_bExecuted: usize = 0xCB8; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_Trapper_WebWall {
        pub const m_vecCastPosition: usize = 0xDB8; //
        pub const m_vecCastPositionNormal: usize = 0xDC4; //
        pub const m_vecEndPosition: usize = 0xDD0; //
        pub const m_vecEndPositionNormal: usize = 0xDDC; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Synth_Pulse_Escape {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbility_Rutger_RocketLauncher_VData {
        pub const m_ImpactParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShootParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Radiance {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_TeleportToGangster {
    }
    // Parent: CCitadel_Ability_PrimaryWeapon
    // Field count: 0
    pub mod CCitadel_Ability_ShivWeapon {
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_PsychicDaggerVData {
        pub const m_MakeDaggersModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CModifierPowerJumpVData {
        pub const m_FloatParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flAirDrag: usize = 0x718; //
        pub const m_flVerticalCameraOffset: usize = 0x71C; //
        pub const m_flVerticalCameraOffsetLerpTime: usize = 0x720; //
        pub const m_flVerticalCameraOffsetBias: usize = 0x724; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 6
    pub mod CCitadel_Ability_Bull_Charge {
        pub const m_anglesCharging: usize = 0xF40; //
        pub const m_flChargeStartTime: usize = 0xF4C; //
        pub const m_flFastChargeEndTime: usize = 0xF50; //
        pub const m_bHitAPlayer: usize = 0xF54; //
        pub const m_bFirstTick: usize = 0xF58; //
        pub const m_vGoalDir: usize = 0xF5C; //
    }
    // Parent: CCitadel_Ability_PrimaryWeapon
    // Field count: 0
    pub mod CCitadel_Ability_PrimaryWeapon_Empty {
    }
    // Parent: CitadelAbilityVData
    // Field count: 57
    pub mod CAbilitySlideVData {
        pub const m_flMinAngleToConsiderASlope: usize = 0x1590; //
        pub const m_flSlideMaxSlopeMaxAccSpeed: usize = 0x1594; //
        pub const m_flSlideMinSlopeMaxAccSpeed: usize = 0x1598; //
        pub const m_flButtonPressWindow: usize = 0x159C; //
        pub const m_flTurnSpeed: usize = 0x15A0; //
        pub const m_flSlideMinSlopeAcceleration: usize = 0x15A4; //
        pub const m_flSlideMaxSlopeAcceleration: usize = 0x15A8; //
        pub const m_flTurnMinAngDiff: usize = 0x15AC; //
        pub const m_flTurnMaxAngDiff: usize = 0x15B0; //
        pub const m_flLandedFlatGroundFrictionGraceTime: usize = 0x15B4; //
        pub const m_flFlatGroundFrictionGraceTime: usize = 0x15B8; //
        pub const m_flFrictionFlatGroundGrace: usize = 0x15BC; //
        pub const m_flFrictionFlatGround: usize = 0x15C0; //
        pub const m_flFrictionMinSlope: usize = 0x15C4; //
        pub const m_flFrictionMaxSlope: usize = 0x15C8; //
        pub const m_flFrictionUphillMinSlope: usize = 0x15CC; //
        pub const m_flFrictionUphillMaxSlope: usize = 0x15D0; //
        pub const m_flLandingSlopeScaleBias: usize = 0x15D4; //
        pub const m_flBoostMinTriggerSpeed: usize = 0x15D8; //
        pub const m_flBoostMaxTriggerSpeed: usize = 0x15DC; //
        pub const m_flBoostMinSpeed: usize = 0x15E0; //
        pub const m_flBoostMaxSpeed: usize = 0x15E4; //
        pub const m_flMinActivationSpeed: usize = 0x15E8; //
        pub const m_flMinSustainSpeed: usize = 0x15EC; //
        pub const m_flSprintBoostSpeed: usize = 0x15F0; //
        pub const m_flDashSlideStartTime: usize = 0x15F4; //
        pub const m_flDashSlideSpeed: usize = 0x15F8; //
        pub const m_flDashSlideFailSpeed: usize = 0x15FC; //
        pub const m_strDashSlideActivate: usize = 0x1600; //
        pub const m_flDashSlideFrictionTime: usize = 0x1610; //
        pub const m_flDashSlideFriction: usize = 0x1614; //
        pub const m_flDashMinActivationSpeed: usize = 0x1618; //
        pub const m_flAccMinSlopeDeg: usize = 0x161C; //
        pub const m_flAccMaxSlopeDeg: usize = 0x1620; //
        pub const m_flAccMinSlopeScale: usize = 0x1624; //
        pub const m_flSlideProbeForwardOffset: usize = 0x1628; //
        pub const m_flSlideActivationProbeForwardOffset: usize = 0x162C; //
        pub const m_flMaxDistanceBetweenProbeSamples: usize = 0x1630; //
        pub const m_flInitialSlideUseForwardProbeTime: usize = 0x1634; //
        pub const m_flCurrentSlopeSampleDistance: usize = 0x1638; //
        pub const m_flSampleVelDiffStdDevScaleCutoff: usize = 0x163C; //
        pub const m_flSlopeFacingAngleToActivate: usize = 0x1640; //
        pub const m_flAirDragAfterJump: usize = 0x1644; //
        pub const m_flAirDragAfterJumpTime: usize = 0x1648; //
        pub const m_flAirDragMaxAngle: usize = 0x164C; //
        pub const m_flAirDragResetTime: usize = 0x1650; //
        pub const m_flLateSlideJumpWindow: usize = 0x1654; //
        pub const m_SlideEffectRemap: usize = 0x1658; //
        pub const m_GetupSpeedCurve: usize = 0x1668; //
        pub const m_flGetupBusyDuration: usize = 0x16A8; //
        pub const m_flSlidingRecoilReduction: usize = 0x16AC; //
        pub const m_cameraSequenceStartSliding: usize = 0x16B0; //
        pub const m_cameraSequenceEndSliding: usize = 0x1738; //
        pub const m_SlideParticle: usize = 0x17C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strStartSound: usize = 0x18A0; //
        pub const m_strLoopingSound: usize = 0x18B0; //
        pub const m_strStopSound: usize = 0x18C0; //
    }
    // Parent: C_BaseEntity
    // Field count: 3
    pub mod C_InfoVisibilityBox {
        pub const m_nMode: usize = 0x564; //
        pub const m_vBoxSize: usize = 0x568; //
        pub const m_bEnabled: usize = 0x574; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Targetdummy_4 {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityTargetPracticeVData {
        pub const m_TargetPracticeSelfModifier: usize = 0x1590; //
        pub const m_TargetPracticeEnemyModifier: usize = 0x15A0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Afterburn {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_MetalSkin {
    }
    // Parent: CCitadel_Ability_TrooperGrenade
    // Field count: 0
    pub mod CCitadel_Ability_TrooperBossGrenade {
    }
    // Parent: None
    // Field count: 1
    pub mod CModifierHandleBase {
        pub const m_hStableHandle: usize = 0x8; //
    }
    // Parent: C_AI_BaseNPC
    // Field count: 7
    pub mod C_AI_CitadelNPC {
        pub const m_bBeamActive: usize = 0xD3C; //
        pub const m_vEyeBeamTarget: usize = 0xD40; //
        pub const m_nPlayerTeamEvent: usize = 0x11C0; //
        pub const m_vecWeakPoints: usize = 0x1210; // C_UtlVectorEmbeddedNetworkVar<WeakPoint_t>
        pub const m_bMinion: usize = 0x1278; //
        pub const m_hLookTarget: usize = 0x127C; //
        pub const m_CCitadelAbilityComponent: usize = 0x1280; //
    }
    // Parent: C_Sprite
    // Field count: 2
    pub mod C_FireSprite {
        pub const m_vecMoveDir: usize = 0x968; //
        pub const m_bFadeFromAbove: usize = 0x974; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ShieldGuy_Ability02 {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Killing_Blow_Glow {
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_Nano_ShadowVData {
        pub const m_ShadowModifier: usize = 0x1590; //
        pub const m_PurgeModifier: usize = 0x15A0; //
        pub const m_EnemyAura: usize = 0x15B0; //
        pub const m_flAuraRadius: usize = 0x15C0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_FissureWallVData {
        pub const m_FriendlyWallParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyWallParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WallTravelSoundLoop: usize = 0x1750; //
        pub const m_WallModifier: usize = 0x1760; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_Upgrade_WeaponPowerForHealthVData {
        pub const m_BuffModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CModifier_Upgrade_ArcaneSurge {
        pub const m_tNextAbilityTriggerWindow: usize = 0x1A0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_HollowPoint_ProcVData {
        pub const m_TracerParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ParticleModifier: usize = 0x718; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_DamageOnHitGround {
    }
    // Parent: None
    // Field count: 2
    pub mod ItemImbuementPair_t {
        pub const m_SourceItemID: usize = 0x30; //
        pub const m_TargetAbilityID: usize = 0x34; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_Upgrade_AmmoScavenger {
        pub const m_hLastOrbTarget: usize = 0xCB8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityTeleportToGangsterVData {
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 0
    pub mod CCitadel_Ability_MedicHeal {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Intrinsic_Base {
    }
    // Parent: C_BaseTrigger
    // Field count: 8
    pub mod C_Precipitation {
        pub const m_flDensity: usize = 0x998; //
        pub const m_flParticleInnerDist: usize = 0x9A8; //
        pub const m_pParticleDef: usize = 0x9B0; //
        pub const m_tParticlePrecipTraceTimer: usize = 0x9D8; //
        pub const m_bActiveParticlePrecipEmitter: usize = 0x9E0; //
        pub const m_bParticlePrecipInitialized: usize = 0x9E1; //
        pub const m_bHasSimulatedSinceLastSceneObjectUpdate: usize = 0x9E2; //
        pub const m_nAvailableSheetSequencesMaxIndex: usize = 0x9E4; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_RegeneratingBulletShieldVData {
        pub const m_ActiveModifier: usize = 0x15D8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_QuickSilverVData {
        pub const m_BuffModifier: usize = 0x668; //
        pub const m_ProcParticle: usize = 0x678; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 1
    pub mod CNPC_SimpleAnimatingAIVData {
        pub const m_sModelName: usize = 0x28; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_TrackedProjectile_Synth_PlasmaFlux {
    }
    // Parent: CBaseDashCastAbilityVData
    // Field count: 1
    pub mod CAbilityCadenceSilenceContraptionsVData {
        pub const m_SilenceContraptionsModifier: usize = 0x1618; //
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 7
    pub mod CPrecipitationVData {
        pub const m_szParticlePrecipitationEffect: usize = 0x28; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flInnerDistance: usize = 0x108; //
        pub const m_nAttachType: usize = 0x10C; //
        pub const m_bBatchSameVolumeType: usize = 0x110; //
        pub const m_nRTEnvCP: usize = 0x114; //
        pub const m_nRTEnvCPComponent: usize = 0x118; //
        pub const m_szModifier: usize = 0x120; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Tengu_Urn {
        pub const m_vLaunchPosition: usize = 0xCA0; //
        pub const m_qLaunchAngle: usize = 0xCAC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_Killing_Blow_GlowVData {
        pub const m_ShivOnlyDeathStatus: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShivOnlyDeathTrail: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strShivOnlyActivateSound: usize = 0x7F8; //
        pub const m_strShivOnlyLoopSound: usize = 0x808; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Astro_Rifle {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Decoy_Tracker {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_ActiveBulletShieldVData {
        pub const m_TempShieldModifier: usize = 0x15D8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_FullSpectrumVData {
        pub const m_DebuffModifier: usize = 0x668; //
        pub const m_BonusDamageModifier: usize = 0x678; //
    }
    // Parent: CCitadel_Modifier_Intrinsic_Base
    // Field count: 0
    pub mod CCitadel_Modifier_OneVsOne {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_ControlPointCapturerAura {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_AccuracyTracker {
        pub const m_flProgress: usize = 0xC0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_GenericPerson_1 {
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityGenericPerson1VData {
    }
    // Parent: CCitadel_Modifier_Base
    // Field count: 0
    pub mod CCitadel_Modifier_Viper_Venom {
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_PerchedPredatorDrag {
        pub const m_qRelativeOffset: usize = 0x130; //
        pub const m_flRelativeDist: usize = 0x13C; //
        pub const m_vecOffsetDir: usize = 0x140; //
        pub const m_hFollowEnt: usize = 0x14C; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CAbilityPsychicPulseVData {
        pub const m_DebuffModifier: usize = 0x1590; //
        pub const m_PulseParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flCastEffectLifetime: usize = 0x1680; //
        pub const m_flConeAngle: usize = 0x1684; //
        pub const m_flConeHalfWidth: usize = 0x1688; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Hornet_Chain_Connection {
        pub const m_vecOrigin: usize = 0xC0; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_Item_Savior_VData {
        pub const m_SaviorModifier: usize = 0x15D8; //
        pub const m_CastParticle: usize = 0x15E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Nano_ShadowPulse_VData {
        pub const m_SatchelParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProc
    // Field count: 3
    pub mod CCitadel_Modifier_BulletFlurry {
        pub const m_nEffectId: usize = 0x230; //
        pub const m_flNextSequenceChange: usize = 0x234; //
        pub const m_nCurrentPose: usize = 0x238; //
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_MedicBulletsVData {
        pub const m_ImpactParticle: usize = 0x768; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ProcSound: usize = 0x848; //
    }
    // Parent: None
    // Field count: 10
    pub mod CBuoyancyHelper {
        pub const m_nFluidType: usize = 0x18; //
        pub const m_flFluidDensity: usize = 0x1C; //
        pub const m_flNeutrallyBuoyantGravity: usize = 0x20; //
        pub const m_flNeutrallyBuoyantLinearDamping: usize = 0x24; //
        pub const m_flNeutrallyBuoyantAngularDamping: usize = 0x28; //
        pub const m_bNeutrallyBuoyant: usize = 0x2C; //
        pub const m_vecFractionOfWheelSubmergedForWheelFriction: usize = 0x30; //
        pub const m_vecWheelFrictionScales: usize = 0x48; //
        pub const m_vecFractionOfWheelSubmergedForWheelDrag: usize = 0x60; //
        pub const m_vecWheelDrag: usize = 0x78; //
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod CFilterAttributeInt {
        pub const m_sAttributeName: usize = 0x5B8; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_Aura_Base {
    }
    // Parent: C_BaseModelEntity
    // Field count: 4
    pub mod C_BaseClientUIEntity {
        pub const m_bEnabled: usize = 0x860; //
        pub const m_DialogXMLName: usize = 0x868; //
        pub const m_PanelClassName: usize = 0x870; //
        pub const m_PanelID: usize = 0x878; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 3
    pub mod C_FuncTrackTrain {
        pub const m_nLongAxis: usize = 0x858; //
        pub const m_flRadius: usize = 0x85C; //
        pub const m_flLineLength: usize = 0x860; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Mirage_SandPhantom_Proc {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilitySleepBombVData {
        pub const m_ExplosionParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AuraModifier: usize = 0x1670; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_RocketLauncher {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_HighAlert {
    }
    // Parent: CCitadelModifierVData
    // Field count: 8
    pub mod CCitadel_Modifier_Bebop_LaserBeamVData {
        pub const m_SlowModifier: usize = 0x638; //
        pub const m_BeamParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BeamParticleLocal: usize = 0x728; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BeamHitParticle: usize = 0x808; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strLaserStartSound: usize = 0x8E8; //
        pub const m_strLaserEndSound: usize = 0x8F8; //
        pub const m_strLaserLoopSound: usize = 0x908; //
        pub const m_strLaserHitSound: usize = 0x918; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Wraith_RapidFire {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Bull_Heal_Target {
        pub const m_flTetherRangeSquared: usize = 0x1D8; //
    }
    // Parent: CPlayer_CameraServices
    // Field count: 1
    pub mod CCitadelPlayer_CameraServices {
        pub const m_hPrevPostProcessingVolume: usize = 0x230; // CHandle<C_PostProcessingVolume>
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 3
    pub mod C_NPC_Trooper {
        pub const m_iLane: usize = 0x1420; //
        pub const m_hTargetedEnemy: usize = 0x1424; //
        pub const m_flHealingChargeParticlePct: usize = 0x1428; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_WreckerSalvageBuffVData {
        pub const m_WeaponBuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Astro_ShotgunBuff {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityAstroRifleVData {
        pub const m_SelfModifier: usize = 0x1590; //
        pub const m_DebuffModifier: usize = 0x15A0; //
        pub const m_SlowModifier: usize = 0x15B0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 5
    pub mod CCitadel_Ability_LashDownStrike {
        pub const m_ImpactTime: usize = 0xD48; //
        pub const m_vDamagePos: usize = 0xD4C; //
        pub const m_PreviewEffect: usize = 0xD5C; //
        pub const m_vStrikeVel: usize = 0xF20; //
        pub const m_flStartHeight: usize = 0xF2C; //
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 13
    pub mod CModifierPsychicLiftVData {
        pub const m_SilenceModifier: usize = 0x718; //
        pub const m_DisarmModifier: usize = 0x728; //
        pub const m_LiftParticle: usize = 0x738; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x818; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strImpactSound: usize = 0x8F8; //
        pub const m_flRiseTime: usize = 0x908; //
        pub const m_flRiseAcc: usize = 0x90C; //
        pub const m_flRiseMaxSpeed: usize = 0x910; //
        pub const m_flRiseDecayFracStart: usize = 0x914; //
        pub const m_flRiseDecayFracEnd: usize = 0x918; //
        pub const m_flSlamAcc: usize = 0x91C; //
        pub const m_flSlamMaxSpeed: usize = 0x920; //
        pub const m_flSlamImpactRadius: usize = 0x924; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CAbilityMeleeVData {
        pub const m_flMeleeInputBufferTime: usize = 0x1590; //
        pub const m_flCollisionDistance: usize = 0x1594; //
        pub const m_flHeavyAttackRequiredHoldTime: usize = 0x1598; //
        pub const m_flLightAttackMaxHoldTime: usize = 0x159C; //
        pub const m_flSideDashDodgeDist: usize = 0x15A0; //
        pub const m_flBackDashDodgeDist: usize = 0x15A4; //
        pub const m_MeleeDamageFlags: usize = 0x15A8; //
        pub const m_strEffectsAttachName: usize = 0x15B0; //
    }
    // Parent: None
    // Field count: 13
    pub mod shard_model_desc_t {
        pub const m_nModelID: usize = 0x8; //
        pub const m_hMaterialBase: usize = 0x10; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_hMaterialDamageOverlay: usize = 0x18; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_solid: usize = 0x20; //
        pub const m_vecPanelSize: usize = 0x24; //
        pub const m_vecStressPositionA: usize = 0x2C; //
        pub const m_vecStressPositionB: usize = 0x34; //
        pub const m_vecPanelVertices: usize = 0x40; //
        pub const m_vInitialPanelVertices: usize = 0x58; //
        pub const m_flGlassHalfThickness: usize = 0x70; //
        pub const m_bHasParent: usize = 0x74; //
        pub const m_bParentFrozen: usize = 0x75; //
        pub const m_SurfacePropStringToken: usize = 0x78; //
    }
    // Parent: None
    // Field count: 1
    pub mod C_SceneEntity__QueuedEvents_t {
        pub const starttime: usize = 0x0; //
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 6
    pub mod C_CitadelItemPickup {
        pub const m_eLootType: usize = 0xB08; //
        pub const m_nCurrencyValue: usize = 0xB0C; //
        pub const m_iszModelName: usize = 0xB10; //
        pub const m_flModelScale: usize = 0xB18; //
        pub const m_hTargetPlayer: usize = 0xB1C; //
        pub const m_flFallRate: usize = 0xB20; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 4
    pub mod CBaseProp {
        pub const m_bModelOverrodeBlockLOS: usize = 0xAF8; //
        pub const m_iShapeType: usize = 0xAFC; //
        pub const m_bConformToCollisionBounds: usize = 0xB00; //
        pub const m_mPreferredCatchTransform: usize = 0xB04; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CModifier_Operative_Revelation_Aura {
    }
    // Parent: CCitadel_Modifier_Intrinsic_Base
    // Field count: 2
    pub mod CCitadel_Modifier_ThrowSandProjectile {
        pub const m_vInitialCastPosition: usize = 0x130; //
        pub const m_iParticleEffect: usize = 0x13C; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ShadowPulse {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_PuddleVData {
        pub const m_puddleAoeDamageFx: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetDamageFx: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityHornetStingVData {
        pub const m_DebuffModifier: usize = 0x1590; //
        pub const m_HitParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RicochetTracerParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 10
    pub mod CModifierStormCloudVData {
        pub const m_ZapFriendly: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DrawFriendly: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AoEFriendly: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZapEnemy: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DrawEnemy: usize = 0x9B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AoEEnemy: usize = 0xA98; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strChannelEndingSoonSound: usize = 0xB78; //
        pub const m_strChannelFinishedSound: usize = 0xB88; //
        pub const m_strDamageRecievedSound: usize = 0xB98; //
        pub const m_strAmbientZapSound: usize = 0xBA8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_MidBossAggroEnemy {
    }
    // Parent: None
    // Field count: 21
    pub mod CBaseModifier {
        pub const m_nSerialNumber: usize = 0x28; //
        pub const m_flLastAppliedTime: usize = 0x2C; //
        pub const m_flCreationTime: usize = 0x30; //
        pub const m_flDuration: usize = 0x34; //
        pub const m_hCaster: usize = 0x38; //
        pub const m_hAbility: usize = 0x3C; //
        pub const m_hAuraProvider: usize = 0x40; //
        pub const m_nAbilitySubclassID: usize = 0x58; //
        pub const m_iAttributes: usize = 0x5C; //
        pub const m_iTeam: usize = 0x5D; //
        pub const m_iStackCount: usize = 0x5E; //
        pub const m_iMaxStackCount: usize = 0x60; //
        pub const m_pVecStackDecayTimes: usize = 0x68; //
        pub const m_eDestroyReason: usize = 0x70; //
        pub const m_bDisabled: usize = 0x71; //
        pub const m_bSuppressSendModifier: usize = 0x72; //
        pub const m_flThinkInterval: usize = 0x74; //
        pub const m_flThinkIntervalStartTime: usize = 0x78; //
        pub const m_flTimeScale: usize = 0x7C; //
        pub const m_pVecTrackedObjects: usize = 0x80; //
        pub const m_hModifierListHandle: usize = 0x88; //
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod CFilterMassGreater {
        pub const m_fFilterMass: usize = 0x5B8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_Fathom_ScaldingSpray_VData {
        pub const m_AuraModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Guiding_Arrow {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_QuickSilver_Buff {
    }
    // Parent: C_BaseModelEntity
    // Field count: 2
    pub mod CItemXP {
        pub const m_timeLaunch: usize = 0x8BC; //
        pub const m_flAttackableTime: usize = 0x8C0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_Synth_Grasp_Victim_VData {
        pub const m_strVictimTetheredSound: usize = 0x638; //
        pub const m_GraspVictimParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityRapidFireVData {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityRiotProtocolVData {
        pub const m_ChargeUpParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WardenBuffModifier: usize = 0x1750; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_Chrono_KineticCarbineVData {
        pub const m_TracerParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FullyChargedParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strFullyCharged: usize = 0x7F8; //
        pub const m_strShotSound: usize = 0x808; //
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod C_CitadelMinimapBoundary {
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 5
    pub mod C_Citadel_Destroyable_Building {
        pub const m_CCitadelAbilityComponent: usize = 0xB00; //
        pub const m_vecWeakPoints: usize = 0xCA0; //
        pub const m_bDestroyed: usize = 0xD08; //
        pub const m_bActive: usize = 0xD09; //
        pub const m_bFinal: usize = 0xD0A; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CItem_RestorativeLocket {
        pub const m_nNumStacks: usize = 0xD60; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_TechUpgrade_SuperAcolyteGloves {
        pub const fl_StoredDamage: usize = 0xCF0; //
    }
    // Parent: CLogicalEntity
    // Field count: 11
    pub mod CPointTemplate {
        pub const m_iszWorldName: usize = 0x560; //
        pub const m_iszSource2EntityLumpName: usize = 0x568; //
        pub const m_iszEntityFilterName: usize = 0x570; //
        pub const m_flTimeoutInterval: usize = 0x578; //
        pub const m_bAsynchronouslySpawnEntities: usize = 0x57C; //
        pub const m_clientOnlyEntityBehavior: usize = 0x580; // PointTemplateClientOnlyEntityBehavior_t
        pub const m_ownerSpawnGroupType: usize = 0x584; // PointTemplateOwnerSpawnGroupType_t
        pub const m_createdSpawnGroupHandles: usize = 0x588; //
        pub const m_SpawnedEntityHandles: usize = 0x5A0; //
        pub const m_ScriptSpawnCallback: usize = 0x5B8; //
        pub const m_ScriptCallbackScope: usize = 0x5C0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityFealtyVData {
        pub const m_TargetModifier: usize = 0x1590; //
    }
    // Parent: CCitadel_Modifier_Disarmed
    // Field count: 0
    pub mod CCitadel_Modifier_ThrowSandDebuff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Decoy_Self_Buff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_HornetSnipeVData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_WingBlast {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_LifeDrainVData {
        pub const m_LifeDrainTargetModifier: usize = 0x1590; //
        pub const m_LifeDrainCasterModifier: usize = 0x15A0; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_BoxingGloveVData {
        pub const m_DebuffModifier: usize = 0x668; //
        pub const m_SwingParticle: usize = 0x678; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x758; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_Item_Bleeding_Bullets_Active {
    }
    // Parent: CCitadel_Modifier_Silenced
    // Field count: 0
    pub mod CCitadel_Modifier_ModDisruptor {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_CloakingDeviceActive {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Fathom_ScaldingSpray {
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CCitadel_Ability_Nano_CatFormVData {
        pub const m_PoofInParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PoofOutParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flModelScale: usize = 0x1750; //
        pub const m_strMeow: usize = 0x1758; //
        pub const m_BuffModifier: usize = 0x1768; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Haze_StackingDamage {
    }
    // Parent: CitadelAbilityVData
    // Field count: 17
    pub mod CCitadel_Ability_Viscous_TelepunchVData {
        pub const m_PortalParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PunchParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WallPunchParticle: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CeilingPunchParticle: usize = 0x1910; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyPortalSound: usize = 0x19F0; //
        pub const m_SelfPortalSound: usize = 0x1A00; //
        pub const m_WindupSound: usize = 0x1A10; //
        pub const m_PunchSound: usize = 0x1A20; //
        pub const m_PunchRollSlowModifier: usize = 0x1A30; //
        pub const m_ImpactModifier: usize = 0x1A40; //
        pub const m_flEnemyPortalTelegraphTime: usize = 0x1A50; //
        pub const m_flSelfPortalTelegraphTime: usize = 0x1A54; //
        pub const m_flWindupTime: usize = 0x1A58; //
        pub const m_flAttackTime: usize = 0x1A5C; //
        pub const m_flGroundTraceOnPlayerHitDistance: usize = 0x1A60; //
        pub const m_flPlayerCheckSphereRadius: usize = 0x1A64; //
    }
    // Parent: CEntityComponent
    // Field count: 3
    pub mod CPathQueryComponent {
        pub const m_vecPathSamplePositions: usize = 0x18; //
        pub const m_vecPathSampleParameters: usize = 0x30; //
        pub const m_vecPathSampleDistances: usize = 0x48; //
    }
    // Parent: C_LightEntity
    // Field count: 0
    pub mod C_LightCapsuleEntity {
    }
    // Parent: C_PointEntity
    // Field count: 5
    pub mod CInfoDynamicShadowHint {
        pub const m_bDisabled: usize = 0x560; //
        pub const m_flRange: usize = 0x564; //
        pub const m_nImportance: usize = 0x568; //
        pub const m_nLightChoice: usize = 0x56C; //
        pub const m_hLight: usize = 0x570; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 5
    pub mod CAbility_Synth_Blitz {
        pub const m_vecSpecialShots: usize = 0xCA0; //
        pub const m_nFastFireBulletsLeft: usize = 0xCB8; //
        pub const m_flBlitzEndTime: usize = 0xCC0; //
        pub const m_bCanApplyTechAmp: usize = 0xCD8; //
        pub const m_bCanLifesteal: usize = 0xCD9; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_TargetPractice {
    }
    // Parent: None
    // Field count: 0
    pub mod CTakeDamageInfoAPI {
    }
    // Parent: None
    // Field count: 0
    pub mod C_BaseEntityAPI {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_HighImpactArmor {
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CAbility_Synth_Pulse_VData {
        pub const m_EscapeModifier: usize = 0x1590; //
        pub const m_DebuffModifier: usize = 0x15A0; //
        pub const m_AoEParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EffectParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChannelParticle: usize = 0x1770; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x1850; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RadiusParticle: usize = 0x1930; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExpireSound: usize = 0x1A10; //
        pub const m_cameraSequenceInSatchel: usize = 0x1A20; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Thumper_3 {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_RocketBarrageVolleyVData {
        pub const m_strFireSound: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ZiplineKnockdownImmune {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CModifierContainmentVictimVData {
        pub const m_AreaParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChainedParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Slow {
    }
    // Parent: None
    // Field count: 10
    pub mod CGlowProperty {
        pub const m_fGlowColor: usize = 0x8; //
        pub const m_iGlowType: usize = 0x30; //
        pub const m_iGlowTeam: usize = 0x34; //
        pub const m_nGlowRange: usize = 0x38; //
        pub const m_nGlowRangeMin: usize = 0x3C; //
        pub const m_glowColorOverride: usize = 0x40; //
        pub const m_bFlashing: usize = 0x44; //
        pub const m_flGlowTime: usize = 0x48; //
        pub const m_flGlowStartTime: usize = 0x4C; //
        pub const m_bGlowing: usize = 0x50; //
    }
    // Parent: C_BaseTrigger
    // Field count: 13
    pub mod C_TriggerPhysics {
        pub const m_gravityScale: usize = 0x998; //
        pub const m_linearLimit: usize = 0x99C; //
        pub const m_linearDamping: usize = 0x9A0; //
        pub const m_angularLimit: usize = 0x9A4; //
        pub const m_angularDamping: usize = 0x9A8; //
        pub const m_linearForce: usize = 0x9AC; //
        pub const m_flFrequency: usize = 0x9B0; //
        pub const m_flDampingRatio: usize = 0x9B4; //
        pub const m_vecLinearForcePointAt: usize = 0x9B8; //
        pub const m_bCollapseToForcePoint: usize = 0x9C4; //
        pub const m_vecLinearForcePointAtWorld: usize = 0x9C8; //
        pub const m_vecLinearForceDirection: usize = 0x9D4; //
        pub const m_bConvertToDebrisWhenPossible: usize = 0x9E0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_FireBeetles_StatStealDebuff {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Mirage_SandPhantom_ProcReady {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 6
    pub mod CCitadel_Ability_Gunslinger_DemonCarbine {
        pub const m_bWantsSlow: usize = 0xCA0; //
        pub const m_flLatchedTimeScaleFracChangeTime: usize = 0xCA4; //
        pub const m_flLatchedTimeScaleFrac: usize = 0xCA8; //
        pub const m_flSpeedBoostEndTime: usize = 0xCAC; //
        pub const m_flShotTimeScaleEndTime: usize = 0xCB0; //
        pub const m_flStoredPowerPct: usize = 0xCB8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 13
    pub mod CCitadel_Modifier_HookTargetVData {
        pub const m_flApproachingWhooshAnticipationTime: usize = 0x638; //
        pub const m_flCloseEnoughDistance: usize = 0x63C; //
        pub const m_flTossUpSpeed: usize = 0x640; //
        pub const m_flReturnDelay: usize = 0x644; //
        pub const m_flReturnDelayAlly: usize = 0x648; //
        pub const m_flReturnSpeed: usize = 0x64C; //
        pub const m_flReturnSpeedNonHero: usize = 0x650; //
        pub const m_flReturnPositionForwardOffset: usize = 0x654; //
        pub const m_SlowModifier: usize = 0x658; //
        pub const m_HookRetrieveParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strRetractSound: usize = 0x748; //
        pub const m_strRetractSoundEnd: usize = 0x758; //
        pub const m_strApproachingWhooshSound: usize = 0x768; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TargetPracticeSelf {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_SilencerProcActive {
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CItem_RestorativeLocket_VData {
        pub const m_CastParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TrailParticle: usize = 0x16B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_CharmedWraps_VData {
        pub const m_SwingParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x748; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_LightEntity
    // Field count: 0
    pub mod C_LightDirectionalEntity {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_SleepBomb_Aura {
    }
    // Parent: C_BaseEntity
    // Field count: 18
    pub mod C_EnvCubemap {
        pub const m_Entity_hCubemapTexture: usize = 0x5E0; //
        pub const m_Entity_bCustomCubemapTexture: usize = 0x5E8; //
        pub const m_Entity_flInfluenceRadius: usize = 0x5EC; //
        pub const m_Entity_vBoxProjectMins: usize = 0x5F0; //
        pub const m_Entity_vBoxProjectMaxs: usize = 0x5FC; //
        pub const m_Entity_bMoveable: usize = 0x608; //
        pub const m_Entity_nHandshake: usize = 0x60C; //
        pub const m_Entity_nEnvCubeMapArrayIndex: usize = 0x610; //
        pub const m_Entity_nPriority: usize = 0x614; //
        pub const m_Entity_flEdgeFadeDist: usize = 0x618; //
        pub const m_Entity_vEdgeFadeDists: usize = 0x61C; //
        pub const m_Entity_flDiffuseScale: usize = 0x628; //
        pub const m_Entity_bStartDisabled: usize = 0x62C; //
        pub const m_Entity_bDefaultEnvMap: usize = 0x62D; //
        pub const m_Entity_bDefaultSpecEnvMap: usize = 0x62E; //
        pub const m_Entity_bIndoorCubeMap: usize = 0x62F; //
        pub const m_Entity_bCopyDiffuseFromDefaultCubemap: usize = 0x630; //
        pub const m_Entity_bEnabled: usize = 0x640; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_Yakuza_Shakedown {
        pub const m_IgnoreChannelSlow: usize = 0xCA0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Viper_DebuffDagger {
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CCitadel_Ability_Wrecker_BoulderGrenadeVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SummonParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SummonReadyParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SummonParticleAttachment: usize = 0x1830; //
        pub const m_ExplodeSound: usize = 0x1838; //
    }
    // Parent: CitadelItemVData
    // Field count: 5
    pub mod CCitadel_Item_CheatDeathVData {
        pub const m_DamagePulseParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DamageTargetParticle: usize = 0x16B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sHealPulseSound: usize = 0x1798; //
        pub const m_sHealAndDamagePulseSound: usize = 0x17A8; //
        pub const m_DeathImmuneModifier: usize = 0x17B8; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_Succor_Move {
        pub const m_bHasPulled: usize = 0xC0; //
        pub const m_bIsPulling: usize = 0xC1; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Upgrade_WeaponPowerForHealth {
    }
    // Parent: CCitadel_Ability_PrimaryWeapon
    // Field count: 0
    pub mod CCitadel_Ability_PrimaryWeapon_Cadence {
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityEmpowerBulletVData {
        pub const m_EmpowerBulletModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_AirLift_Grab {
    }
    // Parent: CCitadelModifier
    // Field count: 9
    pub mod CCitadel_Modifier_ChronoSwap_BubbleMove {
        pub const m_bOtherIsInFrontAtStart: usize = 0xC0; //
        pub const m_vOtherToDest: usize = 0xC4; //
        pub const m_vStart: usize = 0xD0; //
        pub const m_vDest: usize = 0xDC; //
        pub const m_hOther: usize = 0xE8; //
        pub const m_vLastSafePos: usize = 0xEC; //
        pub const m_nNumTicks: usize = 0xF8; //
        pub const m_nTicksLeft: usize = 0xFC; //
        pub const m_nBeamIndex: usize = 0x100; //
    }
    // Parent: CCitadel_Modifier_Base_Buildup
    // Field count: 0
    pub mod CCitadel_Modifier_Silence_Buildup {
    }
    // Parent: CBaseAnimGraph
    // Field count: 6
    pub mod C_Citadel_FissureWall {
        pub const m_vStartPos: usize = 0xAF8; //
        pub const m_vEndPos: usize = 0xB04; //
        pub const m_flStartEmitTime: usize = 0xB10; //
        pub const m_flEndEmitTime: usize = 0xB14; //
        pub const m_bSolid: usize = 0xB18; //
        pub const m_nTouchCount: usize = 0xB1C; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 12
    pub mod CCitadel_Modifier_Mirage_SandPhantom_Proc_VData {
        pub const m_bRollOnceForAllBulletsInAShot: usize = 0x638; //
        pub const m_flMaxBulletsToProcInShot: usize = 0x63C; //
        pub const m_bCanProcMultipleTimesFromSameShot: usize = 0x640; //
        pub const m_bRequiresTargetFilter: usize = 0x641; //
        pub const m_ProcReadyModifier: usize = 0x648; //
        pub const m_PassiveVictimModifier: usize = 0x658; //
        pub const m_ProcReadyParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TracerAdditionParticle: usize = 0x748; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeParticle: usize = 0x828; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_OnBulletRolledProcSound: usize = 0x908; //
        pub const m_ProcSound: usize = 0x918; //
        pub const m_ExplodeSound: usize = 0x928; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_EmpowerBullet {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_ViscousBall {
        pub const m_nDirectionParticleIndex: usize = 0xC0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_PassiveBeefyVData {
        pub const m_HealParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_IntensifyingClip {
        pub const m_LastThinkTime: usize = 0x130; //
        pub const m_flSpinUpTime: usize = 0x134; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_Ricochet_Proc {
    }
    // Parent: CBaseAnimGraph
    // Field count: 4
    pub mod CPropAnimatingBreakable {
        pub const m_stages: usize = 0xAF8; //
        pub const m_OnTakeDamage: usize = 0xB10; //
        pub const m_OnFinalBreak: usize = 0xB38; //
        pub const m_OnStageAdvanced: usize = 0xB60; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_Operative_Revelation_Target_VData {
        pub const m_DebuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ShakedownPulse {
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CAbilityStickyBombVData {
        pub const m_BombAttachedModifier: usize = 0x1590; //
        pub const m_KillCheckModifier: usize = 0x15A0; //
        pub const m_CastBombParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flPostRangeGravityScale: usize = 0x1690; //
        pub const m_flAllyCollideRadius: usize = 0x1694; //
        pub const m_flBombDragStartRange: usize = 0x1698; //
        pub const m_flBombDragStartValue: usize = 0x169C; //
        pub const m_flBombDragEndValue: usize = 0x16A0; //
        pub const m_flAllyTargetRangeMult: usize = 0x16A4; //
    }
    // Parent: C_BaseEntity
    // Field count: 25
    pub mod C_PointValueRemapper {
        pub const m_bDisabled: usize = 0x560; //
        pub const m_bDisabledOld: usize = 0x561; //
        pub const m_bUpdateOnClient: usize = 0x562; //
        pub const m_nInputType: usize = 0x564; //
        pub const m_hRemapLineStart: usize = 0x568; //
        pub const m_hRemapLineEnd: usize = 0x56C; //
        pub const m_flMaximumChangePerSecond: usize = 0x570; //
        pub const m_flDisengageDistance: usize = 0x574; //
        pub const m_flEngageDistance: usize = 0x578; //
        pub const m_bRequiresUseKey: usize = 0x57C; //
        pub const m_nOutputType: usize = 0x580; //
        pub const m_hOutputEntities: usize = 0x588; //
        pub const m_nHapticsType: usize = 0x5A0; //
        pub const m_nMomentumType: usize = 0x5A4; //
        pub const m_flMomentumModifier: usize = 0x5A8; //
        pub const m_flSnapValue: usize = 0x5AC; //
        pub const m_flCurrentMomentum: usize = 0x5B0; //
        pub const m_nRatchetType: usize = 0x5B4; //
        pub const m_flRatchetOffset: usize = 0x5B8; //
        pub const m_flInputOffset: usize = 0x5BC; //
        pub const m_bEngaged: usize = 0x5C0; //
        pub const m_bFirstUpdate: usize = 0x5C1; //
        pub const m_flPreviousValue: usize = 0x5C4; //
        pub const m_flPreviousUpdateTickTime: usize = 0x5C8; //
        pub const m_vecPreviousTestPoint: usize = 0x5CC; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Spin {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CModifierQuarantineVData {
        pub const m_BubbleParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BubbleExplodeParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SilenceModifier: usize = 0x7F8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Neutral_Debuff_Pushback {
    }
    // Parent: C_Citadel_BreakblePropPickup
    // Field count: 1
    pub mod C_Citadel_BreakblePropGoldPickup {
        pub const m_iGoldReward: usize = 0xB28; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_Item_Intensifying_Clip {
        pub const m_flSpinUpTime: usize = 0xCF0; //
    }
    // Parent: C_EnvCubemap
    // Field count: 0
    pub mod C_EnvCubemapBox {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 2
    pub mod CCitadelViscousBallVData {
        pub const m_sModelName: usize = 0x28; //
        pub const m_flPhysicsRadius: usize = 0x108; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_WeaponUpgrade_HeadshotBooster_VData {
        pub const m_HeadShotVictimSound: usize = 0x15D8; //
        pub const m_HeadShotConfirmationSound: usize = 0x15E8; //
    }
    // Parent: C_NPC_Trooper
    // Field count: 3
    pub mod C_NPC_TrooperBoss {
        pub const m_CCitadelPlayerClipComponent: usize = 0x1468; //
        pub const m_flFadeOutStart: usize = 0x1494; //
        pub const m_flFadeOutEnd: usize = 0x1498; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ThrowSand {
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbilityBouncePadVData {
        pub const m_BounceModifier: usize = 0x1590; //
        pub const m_AllyBounceModifier: usize = 0x15A0; //
        pub const m_SpeedOnLandModifier: usize = 0x15B0; //
        pub const m_NoBounceModifier: usize = 0x15C0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_RocketBarrageVolley {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 6
    pub mod CCitadel_Ability_Melee_Base {
        pub const m_bUsingThisMelee: usize = 0xCA0; //
        pub const m_bUsingMeleeTagActive: usize = 0xCA1; //
        pub const m_bHitWithThisAttack: usize = 0xCA2; //
        pub const m_flLastActivateTime: usize = 0xCA4; //
        pub const m_flNextAttackAllowedTime: usize = 0xCA8; //
        pub const m_flAttackTriggeredTime: usize = 0xCAC; //
    }
    // Parent: CCitadel_Modifier_ChainLightningEffect
    // Field count: 0
    pub mod CCitadel_Modifier_PowerSurge_ChainLightning {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_FullSpectrumDamage {
    }
    // Parent: C_BaseEntity
    // Field count: 5
    pub mod C_SoundOpvarSetPointBase {
        pub const m_iszStackName: usize = 0x560; //
        pub const m_iszOperatorName: usize = 0x568; //
        pub const m_iszOpvarName: usize = 0x570; //
        pub const m_iOpvarIndex: usize = 0x578; //
        pub const m_bUseAutoCompare: usize = 0x57C; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_TrapperPoisonJar_Aura {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_TechDamagePulse {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Tier3BossInvuln {
    }
    // Parent: C_Breakable
    // Field count: 0
    pub mod C_PhysBox {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_Gunslinger_WallStunVData {
        pub const m_ProcEffect: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StunModifier: usize = 0x718; //
        pub const m_CasterMarkTriggerSound: usize = 0x728; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_PristineEmblem_VData {
        pub const m_TracerParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ParticleModifier: usize = 0x718; //
    }
    // Parent: None
    // Field count: 0
    pub mod CPathSimpleAPI {
    }
    // Parent: C_BaseTrigger
    // Field count: 0
    pub mod C_CitadelShopTunnelTrigger {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_ModDisruptor {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_FrenzyAura {
    }
    // Parent: CLogicalEntity
    // Field count: 3
    pub mod CBaseFilter {
        pub const m_bNegated: usize = 0x560; //
        pub const m_OnPass: usize = 0x568; //
        pub const m_OnFail: usize = 0x590; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 15
    pub mod CCitadel_Ability_Nano_Pounce_VData {
        pub const m_LeapModifier: usize = 0x1590; //
        pub const m_ActiveBuff: usize = 0x15A0; //
        pub const m_SlowModifier: usize = 0x15B0; //
        pub const m_DoublePounceModifier: usize = 0x15C0; //
        pub const m_AttackParticle: usize = 0x15D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FlashParticle: usize = 0x16B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1790; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSlowParticle: usize = 0x1870; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PrimaryHitParticle: usize = 0x1950; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AttackSound: usize = 0x1A30; //
        pub const m_strExplodeSound: usize = 0x1A40; //
        pub const m_flAttackTimePhase01: usize = 0x1A50; //
        pub const m_flAttackTimePhase02: usize = 0x1A54; //
        pub const m_flAllyMinTargetRange: usize = 0x1A58; //
        pub const m_flTargetVerticalOffset: usize = 0x1A5C; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CAbilityHornetChainVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExplodeSound: usize = 0x1670; //
        pub const m_ChainModifier: usize = 0x1680; //
        pub const m_DisarmModifier: usize = 0x1690; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 11
    pub mod CCitadel_Ability_GuidedArrow {
        pub const m_hProjectile: usize = 0xCA8; //
        pub const m_flArrowSpeed: usize = 0xCAC; //
        pub const m_flSnapAnglesBackTime: usize = 0xCB0; //
        pub const m_nBonusTechPower: usize = 0xCB4; //
        pub const m_flCastTime: usize = 0xCB8; //
        pub const m_bNeedsExplosion: usize = 0xCBC; //
        pub const m_vProjectileRemovedOrigin: usize = 0xCC0; //
        pub const m_angCasterAnglesAtCastTime: usize = 0xCCC; //
        pub const m_flTravelDistance: usize = 0xCD8; //
        pub const m_bInKillFlow: usize = 0xCDC; //
        pub const m_flProjectileTurnVel: usize = 0xCE0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_FireBombVData {
        pub const m_ChargeParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_GroundParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Ability_Melee_Base
    // Field count: 10
    pub mod CCitadel_Ability_HoldMelee {
        pub const m_flParryWindowEndTime: usize = 0xD20; //
        pub const m_flNextParryTime: usize = 0xD24; //
        pub const m_flStateStartTime: usize = 0xD28; //
        pub const m_flDashStartTime: usize = 0xD2C; //
        pub const m_eCurrentAttackState: usize = 0xD30; //
        pub const m_eCurrentAttackType: usize = 0xD34; //
        pub const m_vAirDashDir: usize = 0xD38; //
        pub const m_bAttackStartedWhileSliding: usize = 0xD44; //
        pub const m_bCreatedChargeEffects: usize = 0xD45; //
        pub const m_angForced: usize = 0xD48; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadelModifierApexWatcherVData {
        pub const m_BuffModifier: usize = 0x638; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 22
    pub mod CAbilityJumpVData {
        pub const m_flShootingLockoutAfterJump: usize = 0x1590; //
        pub const m_DashJumpParticle: usize = 0x1598; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AirJumpParticle: usize = 0x1678; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WallJumpParticle: usize = 0x1758; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AirJumpExecutedSound: usize = 0x1838; //
        pub const m_flMantleRefundWindow: usize = 0x1848; //
        pub const m_flZiplineRefundWindow: usize = 0x184C; //
        pub const m_flLateJumpGraceWindow: usize = 0x1850; //
        pub const m_flMaxSpeedDelta: usize = 0x1854; //
        pub const m_strDashJumpActivate: usize = 0x1858; //
        pub const m_flDashJumpStartTime: usize = 0x1868; //
        pub const m_flDashJumpEndTime: usize = 0x186C; //
        pub const m_flDashJumpDistanceInMeters: usize = 0x1870; //
        pub const m_flDashJumpVerticalSpeed: usize = 0x1878; //
        pub const m_flDashJumpMissMaxSpeed: usize = 0x187C; //
        pub const m_flDashJumpMantleDisableTime: usize = 0x1880; //
        pub const m_WallJumpExecutedSound: usize = 0x1888; //
        pub const m_flCollidedWallMaxDist: usize = 0x1898; //
        pub const m_flRemapSpeedToWallJumpVelocityDist: usize = 0x189C; //
        pub const m_flWallJumpNormalSpeed: usize = 0x18AC; //
        pub const m_WallJumpAirDragCurve: usize = 0x18B0; //
        pub const m_flMaxWallYawOffset: usize = 0x18F0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_Objective_HealthGrowthVData {
        pub const m_iGrowthPerMinute: usize = 0x638; //
        pub const m_flTickRate: usize = 0x63C; //
        pub const m_iGrowthStartTimeInMinutes: usize = 0x640; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Objective_BulletReistVData {
        pub const m_BulletResist: usize = 0x638; //
        pub const m_BulletResistReductionPerHero: usize = 0x63C; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 7
    pub mod CCitadel_Modifier_Basic_RangedArmorBonusVData {
        pub const m_flBulletResistancePctMin: usize = 0x638; //
        pub const m_flBulletResistancePctMax: usize = 0x63C; //
        pub const m_flTechResistancePctMin: usize = 0x640; //
        pub const m_flTechResistancePctMax: usize = 0x644; //
        pub const m_flRangeMin: usize = 0x648; //
        pub const m_flRangeMax: usize = 0x64C; //
        pub const m_flInvulnRange: usize = 0x650; //
    }
    // Parent: None
    // Field count: 6
    pub mod WeakPoint_t {
        pub const m_bRegistered: usize = 0x3C; //
        pub const m_hOuter: usize = 0x40; //
        pub const m_nCritHitGroup: usize = 0x44; //
        pub const m_nBodyGroup: usize = 0x48; //
        pub const m_bPermanentlyBroken: usize = 0x4C; //
        pub const m_nBrokenBodygroupIndex: usize = 0x50; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 1
    pub mod C_NPC_SimpleAnimatingAI {
        pub const m_hEnemy: usize = 0xAF8; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_Empty {
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CCitadel_Ability_IcePathVData {
        pub const m_IcePathModifier: usize = 0x1590; //
        pub const m_flMomentumDecayRate: usize = 0x15A0; //
        pub const m_flMomentumWeight: usize = 0x15A4; //
        pub const m_flMaxPitchChange: usize = 0x15A8; //
        pub const m_flMaxPitchUp: usize = 0x15AC; //
        pub const m_flMaxPitchDown: usize = 0x15B0; //
        pub const m_flMaxHeight: usize = 0x15B4; //
        pub const m_flForwardAngleBias: usize = 0x15B8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CCitadel_Ability_HealthSwapVData {
        pub const m_SwapParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SilenceExplodeParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SwapModifier: usize = 0x1750; //
        pub const m_PreCastModifier: usize = 0x1760; //
        pub const m_BuffModifier: usize = 0x1770; //
        pub const m_SilenceModifier: usize = 0x1780; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_SiphonBullets_RestoreHealth {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_EscalatingExposureProcWatcher {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_CanDamageMidBoss {
    }
    // Parent: C_BaseTrigger
    // Field count: 15
    pub mod CCitadelControlPointTrigger {
        pub const m_flInitialRadius: usize = 0x998; //
        pub const m_flEndRadius: usize = 0x99C; //
        pub const m_flProgress: usize = 0x9A0; //
        pub const m_flCaptureTime: usize = 0x9A4; //
        pub const m_hUnlockPrereq: usize = 0x9A8; //
        pub const m_bAvailable: usize = 0x9AC; //
        pub const m_bIsBeingCaptured: usize = 0x9AD; //
        pub const m_bIsBeingBlocked: usize = 0x9AE; //
        pub const m_flLastTouchedTime: usize = 0x9B8; //
        pub const m_vecBeamTarget: usize = 0x9BC; //
        pub const m_vecBeamStart: usize = 0x9C8; //
        pub const m_nFXProgressBeam: usize = 0x9D4; //
        pub const m_strUnlockPrereq: usize = 0x9D8; //
        pub const m_strBeamStart: usize = 0x9E0; //
        pub const m_strBeamTarget: usize = 0x9E8; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_SingleTargetStun {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Gunslinger_DemonMark {
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_Nano_PredatoryStatueTargetVData {
        pub const m_strLaserHitSound: usize = 0x638; //
        pub const m_strLaserStartSound: usize = 0x648; //
        pub const m_strLaserLoopSound: usize = 0x658; //
        pub const m_DebuffModifier: usize = 0x668; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_PsychicDagger_MakeDaggers {
        pub const m_nSatVolumeIndex: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_Bull_Leap_Boosting_CrashVData {
        pub const m_DragModifier: usize = 0x638; //
        pub const m_CrashTrailParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flCollideRadius: usize = 0x728; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 40
    pub mod CCitadel_Ability_PrimaryWeapon {
        pub const m_flNextPrimaryAttack: usize = 0xCA0; //
        pub const m_iClip: usize = 0xCA4; //
        pub const m_iBonusClip: usize = 0xCA8; //
        pub const m_flSpreadPenalty: usize = 0xCAC; //
        pub const m_flZoomTime: usize = 0xCB0; //
        pub const m_flZoomOutTime: usize = 0xCB4; //
        pub const m_iSpreadIndex: usize = 0xCB8; //
        pub const m_nShotRecoilIndex: usize = 0xCBA; //
        pub const m_flNextShotRecoilRecoveryTime: usize = 0xCBC; //
        pub const m_bIsZoomed: usize = 0xCC0; //
        pub const m_nBurstShotsRemaining: usize = 0xCC1; //
        pub const m_nShotNumber: usize = 0xCC4; //
        pub const m_bInReload: usize = 0xCC8; //
        pub const m_bSingleShotReloadFirstBullet: usize = 0xCC9; //
        pub const m_reloadQueuedStartTime: usize = 0xCCC; //
        pub const m_flReloadAvailableTime: usize = 0xCD0; //
        pub const m_bCanActiveReload: usize = 0xCD4; //
        pub const m_flLastAttackTime: usize = 0xCD8; //
        pub const m_flNextAttackDelayStartTime: usize = 0xCDC; //
        pub const m_flNextAttackDelayEndTime: usize = 0xCE0; //
        pub const m_flAttackDelayPauseTotalTime: usize = 0xCE4; //
        pub const m_flAttackDelayPauseEndTime: usize = 0xCE8; //
        pub const m_eNextAttackDelayReason: usize = 0xCEC; //
        pub const m_bInputPressedWhileSelected: usize = 0xCF0; //
        pub const m_eActiveFireMode: usize = 0xCF4; //
        pub const m_angRecoilAngles: usize = 0xCF8; //
        pub const m_angRecoilToAdd: usize = 0xD04; //
        pub const m_angRecoilRecovery: usize = 0xD10; //
        pub const m_flRecoilStartTime: usize = 0xD1C; //
        pub const m_flRecoilRecoverySpeed: usize = 0xD20; //
        pub const m_flAddApproachSpeed: usize = 0xD24; //
        pub const m_bFireBackwards: usize = 0xD28; //
        pub const m_currentSpread: usize = 0xD2C; //
        pub const m_currentMaxSpread: usize = 0xD30; //
        pub const m_currentFireSpread: usize = 0xD34; //
        pub const m_flCurrentSpinRate: usize = 0xD38; //
        pub const m_fFireDuration: usize = 0xD40; //
        pub const m_bFireOnEmpty: usize = 0xD45; //
        pub const m_bHasReleasedForSemiAuto: usize = 0xD46; //
        pub const m_flNextDisarmSound: usize = 0xD48; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_StompDebuff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Surging_PowerVData {
        pub const m_BerserkerSound: usize = 0x638; //
        pub const m_ModifierActiveDisplay: usize = 0x648; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Neutral_Debuff_PushbackVData {
        pub const m_flPushSpeed: usize = 0x638; //
        pub const m_flPushRange: usize = 0x63C; //
    }
    // Parent: CInfoDynamicShadowHint
    // Field count: 2
    pub mod CInfoDynamicShadowHintBox {
        pub const m_vBoxMins: usize = 0x578; //
        pub const m_vBoxMaxs: usize = 0x584; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Projectile_Mirage_Tornado {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Gunslinger_KnockbackBlast {
        pub const m_vecKnockbackDirection: usize = 0xCA0; //
        pub const m_vecKnockbackedUnits: usize = 0xCB0; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 7
    pub mod CModifierAirLiftGrabVData {
        pub const m_GrabEffect: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flLiftHorizontal: usize = 0x718; //
        pub const m_flLiftHeight: usize = 0x71C; //
        pub const m_flFollowDampingFactor: usize = 0x720; //
        pub const m_flFollowDistance: usize = 0x724; //
        pub const m_flAllyGrabCancelTime: usize = 0x728; //
        pub const m_flAllyPossibleStuckDistance: usize = 0x72C; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_Wrecker_Ultimate_GrabEnemy {
        pub const m_vHoldOffset: usize = 0xC0; //
        pub const m_flLastTouchTime: usize = 0xCC; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_FissureWall {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_InstantReloadVData {
        pub const m_ReloadParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_TeleportToObjective {
        pub const m_vDest: usize = 0xC0; //
        pub const m_angDestAngles: usize = 0xCC; //
        pub const m_vDestVelocity: usize = 0xD8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PlayerDisconnected {
    }
    // Parent: C_BaseModelEntity
    // Field count: 4
    pub mod C_Citadel_Ice_Path_Shard_Physics {
        pub const m_ShardDesc: usize = 0x858; //
        pub const m_qForward: usize = 0x890; //
        pub const m_flStartTime: usize = 0x89C; //
        pub const m_flEndTime: usize = 0x8A0; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_UtilityUpgrade_RocketBoots {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_CopyUlt {
    }
    // Parent: CitadelAbilityVData
    // Field count: 7
    pub mod CAbilityPerchedPredatorVData {
        pub const m_ExplodeBaseParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeFriendlyParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeEnemyParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExplodeSound: usize = 0x1830; //
        pub const m_ModifierDragEnemy: usize = 0x1840; //
        pub const m_flOnHitDetonateTimer: usize = 0x1850; //
        pub const m_flTraceTravelRadius: usize = 0x1854; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_UppercutClipSize {
        pub const m_nPreClipSize: usize = 0xF8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_SpeedBoostVData {
        pub const m_flMoveSpeedBoost: usize = 0x638; //
    }
    // Parent: C_SoundOpvarSetPointEntity
    // Field count: 0
    pub mod C_SoundOpvarSetPathCornerEntity {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CBaseTriggerAbilityVData {
        pub const m_AbilityToTrigger: usize = 0x1590; //
        pub const m_flMinCancelTime: usize = 0x15A0; //
        pub const m_eHintFeatureToMarkUsedOnTrigger: usize = 0x15A4; //
    }
    // Parent: C_BaseEntity
    // Field count: 18
    pub mod C_EnvCubemapFog {
        pub const m_flEndDistance: usize = 0x560; //
        pub const m_flStartDistance: usize = 0x564; //
        pub const m_flFogFalloffExponent: usize = 0x568; //
        pub const m_bHeightFogEnabled: usize = 0x56C; //
        pub const m_flFogHeightWidth: usize = 0x570; //
        pub const m_flFogHeightEnd: usize = 0x574; //
        pub const m_flFogHeightStart: usize = 0x578; //
        pub const m_flFogHeightExponent: usize = 0x57C; //
        pub const m_flLODBias: usize = 0x580; //
        pub const m_bActive: usize = 0x584; //
        pub const m_bStartDisabled: usize = 0x585; //
        pub const m_flFogMaxOpacity: usize = 0x588; //
        pub const m_nCubemapSourceType: usize = 0x58C; //
        pub const m_hSkyMaterial: usize = 0x590; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_iszSkyEntity: usize = 0x598; //
        pub const m_hFogCubemapTexture: usize = 0x5A0; //
        pub const m_bHasHeightFogEnd: usize = 0x5A8; //
        pub const m_bFirstTime: usize = 0x5A9; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityGenericPerson2VData {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Afterburn_DOT_VData {
        pub const m_sAfterburnParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Fervor_Bonuses {
        pub const m_nBonusesParticle: usize = 0xC0; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 3
    pub mod CCitadel_Modifier_Rutger_Pulse_Aura {
        pub const m_flStartRadius: usize = 0xF0; //
        pub const m_flEndRadius: usize = 0xF4; //
        pub const m_flSpreadDuration: usize = 0xF8; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_DoubleJump {
        pub const m_nTickJumped: usize = 0xCB8; //
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CItemSilenceGlyph {
        pub const m_vecProjectileHitTargets: usize = 0xCB8; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PayloadCarrier {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Dust_Storm_Aura_Apply {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierChargedTacklePrepareVData {
        pub const m_PrepareParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_MeleeParry {
        pub const m_nActiveFX: usize = 0xCA0; //
        pub const m_flParryStartTime: usize = 0xCA4; //
        pub const m_bAttackParried: usize = 0xCA8; //
        pub const m_flParrySuccessTime: usize = 0xCAC; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_BerserkerDamageStack {
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProc
    // Field count: 1
    pub mod CCitadel_Modifier_ExplosiveBullets {
        pub const m_BuffedShotId: usize = 0x1F8; //
    }
    // Parent: None
    // Field count: 4
    pub mod DynamicAbilityValues_t {
        pub const m_SourceAbilityID: usize = 0x30; //
        pub const m_TargetAbilityID: usize = 0x34; //
        pub const m_eValType: usize = 0x38; //
        pub const m_flValue: usize = 0x3C; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_MetalSkin {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Fathom_ScaldingSpray_WeaponDamage {
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CCitadel_Ability_Spinning_BladeVData {
        pub const m_DebuffModifier: usize = 0x1590; //
        pub const m_CatchIndicator: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CatchParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strThrowSound: usize = 0x1760; //
        pub const m_strReturnSound: usize = 0x1770; //
        pub const m_strCatchSound: usize = 0x1780; //
        pub const m_strFailSound: usize = 0x1790; //
        pub const m_strHitSound: usize = 0x17A0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 36
    pub mod CCitadel_Ability_Tengu_AirLiftVData {
        pub const m_FlyingModifier: usize = 0x1590; //
        pub const m_GrabModifier: usize = 0x15A0; //
        pub const m_HoldBombModifier: usize = 0x15B0; //
        pub const m_DroppedBuffModifier: usize = 0x15C0; //
        pub const m_ExplodingAllyModifier: usize = 0x15D0; //
        pub const m_AoEModifier: usize = 0x15E0; //
        pub const m_InitialExplodeParticle: usize = 0x15F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HoldBombEffect: usize = 0x16D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExplodeSound: usize = 0x17B0; //
        pub const m_flAirDrag: usize = 0x17C0; //
        pub const m_flMaxFallSpeed: usize = 0x17C4; //
        pub const m_flTargetAirSpeedFast: usize = 0x17C8; //
        pub const m_flTargetAirSpeedBase: usize = 0x17CC; //
        pub const m_flSprintMult: usize = 0x17D0; //
        pub const m_flAcceleration: usize = 0x17D4; //
        pub const m_flDecceleration: usize = 0x17D8; //
        pub const m_flAirSideSpeedPercent: usize = 0x17DC; //
        pub const m_flBoostEndVerticalSpeed: usize = 0x17E0; //
        pub const m_flBoostSpeedUp: usize = 0x17E4; //
        pub const m_flCrouchLaunchReduction: usize = 0x17E8; //
        pub const m_flMinFlyHeight: usize = 0x17EC; //
        pub const m_flMaxFlyHeight: usize = 0x17F0; //
        pub const m_flMaxPitchUp: usize = 0x17F4; //
        pub const m_flMaxPitchDown: usize = 0x17F8; //
        pub const m_flAllyDelayedBoostTime: usize = 0x17FC; //
        pub const m_flChannelingAirDrag: usize = 0x1800; //
        pub const m_flChannelingMaxFallSpeed: usize = 0x1804; //
        pub const m_flBombReleaseSpeed: usize = 0x1808; //
        pub const m_flBombReleasePitch: usize = 0x180C; //
        pub const m_flBombDropReleaseOffset: usize = 0x1810; //
        pub const m_flHoldBombOffsetX: usize = 0x1814; //
        pub const m_flHoldBombOffsetY: usize = 0x1818; //
        pub const m_flHoldBombOffsetZ: usize = 0x181C; //
        pub const m_flAnglePitchBias: usize = 0x1820; //
        pub const m_flTrackAmount: usize = 0x1824; //
        pub const m_flMoveCollideSpeed: usize = 0x1828; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_ChronoSwap_BubbleMoveVData {
        pub const m_BeamParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HealParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DamageParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Stabilizing_Tripod_Self_Debuff {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Hero_Clone {
        pub const m_bMimicOwner: usize = 0xC0; //
    }
    // Parent: None
    // Field count: 17
    pub mod CCollisionProperty {
        pub const m_collisionAttribute: usize = 0x10; //
        pub const m_vecMins: usize = 0x40; //
        pub const m_vecMaxs: usize = 0x4C; //
        pub const m_usSolidFlags: usize = 0x5A; //
        pub const m_nSolidType: usize = 0x5B; //
        pub const m_triggerBloat: usize = 0x5C; //
        pub const m_nSurroundType: usize = 0x5D; //
        pub const m_CollisionGroup: usize = 0x5E; //
        pub const m_nEnablePhysics: usize = 0x5F; //
        pub const m_flBoundingRadius: usize = 0x60; //
        pub const m_vecSpecifiedSurroundingMins: usize = 0x64; //
        pub const m_vecSpecifiedSurroundingMaxs: usize = 0x70; //
        pub const m_vecSurroundingMaxs: usize = 0x7C; //
        pub const m_vecSurroundingMins: usize = 0x88; //
        pub const m_vCapsuleCenter1: usize = 0x94; //
        pub const m_vCapsuleCenter2: usize = 0xA0; //
        pub const m_flCapsuleRadius: usize = 0xAC; //
    }
    // Parent: None
    // Field count: 8
    pub mod CNetworkedSequenceOperation {
        pub const m_hSequence: usize = 0x8; //
        pub const m_flPrevCycle: usize = 0xC; //
        pub const m_flCycle: usize = 0x10; //
        pub const m_flWeight: usize = 0x14; //
        pub const m_bSequenceChangeNetworked: usize = 0x1C; //
        pub const m_bDiscontinuity: usize = 0x1D; //
        pub const m_flPrevCycleFromDiscontinuity: usize = 0x20; //
        pub const m_flPrevCycleForAnimEventDetection: usize = 0x24; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_Cadence_GrandFinaleAOEVData {
        pub const m_AuraParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Operative_UmbrellaManeuver_AirHang {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Targetdummy_Inherent {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Haunt_Damage_VData {
        pub const m_sAfterburnParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityShivDeferDamageVData {
        pub const m_ActiveCastParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flDeferredDamageApplicationInterval: usize = 0x1670; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Bebop_Hook_BulletAmp {
    }
    // Parent: CCitadel_Modifier_StatStealBase
    // Field count: 0
    pub mod CCitadel_Modifier_Arcane_Eater_Watcher {
    }
    // Parent: CTier3BossAbility
    // Field count: 0
    pub mod CCitadel_Ability_Weapon_BossTier3 {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_CheaterCurseVData {
        pub const m_CursedModel: usize = 0x638; //
        pub const m_flModelScale: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CBaseLockonAbilityVData {
        pub const m_TargetModifier: usize = 0x1590; //
        pub const m_strApplyLockonStack: usize = 0x15A0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CAbility_Operative_UmbrellaManeuver {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Synth_Grasp_Caster {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_RiotProtocol {
        pub const m_bActive: usize = 0xCA0; //
    }
    // Parent: CCitadel_Modifier_Intrinsic_BaseVData
    // Field count: 1
    pub mod CCitadel_Modifier_ThrowSandProjectileVData {
        pub const m_DebuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Intimidated_Debuff {
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 6
    pub mod CCitadel_Modifier_UltCombo_TargetVData {
        pub const m_flTargetPosDistance: usize = 0x718; //
        pub const m_flTargetPosRange: usize = 0x71C; //
        pub const m_flPullSpeedMin: usize = 0x720; //
        pub const m_flPullSpeedMax: usize = 0x724; //
        pub const m_flPullDistanceMin: usize = 0x728; //
        pub const m_flPullDistanceMax: usize = 0x72C; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Astro_Rifle_DebuffVData {
        pub const m_SlowModifier: usize = 0x638; //
        pub const m_strTargetHitSound: usize = 0x648; //
    }
    // Parent: CitadelItemVData
    // Field count: 11
    pub mod CCitadel_Upgrade_MagicCarpetVData {
        pub const m_SummonParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FlyingCarpetModifier: usize = 0x16B8; //
        pub const m_SummonFlyingCarpetModifier: usize = 0x16C8; //
        pub const m_SummonFlyingCarpetVisualModifier: usize = 0x16D8; //
        pub const m_FlyingCarpetVisualModifier: usize = 0x16E8; //
        pub const m_ShieldModifier: usize = 0x16F8; //
        pub const m_flSummonVisualDuration: usize = 0x1708; //
        pub const m_flBurstSpeedBonus: usize = 0x170C; //
        pub const m_flBurstSpeedMin: usize = 0x1710; //
        pub const m_flBurstSpeedDuration: usize = 0x1714; //
        pub const m_flMinDistanceAboveGround: usize = 0x1718; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_FullSpectrum {
    }
    // Parent: CCitadel_Modifier_Tier3Boss_Base
    // Field count: 18
    pub mod CCitadel_Modifier_Tier3Boss_LaserBeam {
        pub const m_flSoundStartTime: usize = 0xC8; //
        pub const m_vStart: usize = 0xD0; //
        pub const m_vEnd: usize = 0xDC; //
        pub const m_vPrevEnd: usize = 0xE8; //
        pub const m_flAngleBetweenTrace: usize = 0xF4; //
        pub const m_flPlayerDamagePerTick: usize = 0xF8; //
        pub const m_flNPCDamagePerTick: usize = 0xFC; //
        pub const m_flNextDamageTick: usize = 0x100; //
        pub const m_vecEntitiesHit: usize = 0x108; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_flDamageTickRate: usize = 0x120; //
        pub const m_flLastShakeTime: usize = 0x124; //
        pub const m_bSweepRightFirst: usize = 0x128; //
        pub const m_vecBeamTarget: usize = 0x12C; //
        pub const m_flLastBeamUpdateTime: usize = 0x138; //
        pub const m_vecEnemyPosition: usize = 0x13C; //
        pub const m_nTrackingIndex: usize = 0x148; //
        pub const m_bPreviewMode: usize = 0x14C; //
        pub const m_hAttachment: usize = 0x14D; //
    }
    // Parent: None
    // Field count: 13
    pub mod CModifierProperty {
        pub const __m_pChainEntity: usize = 0x8; //
        pub const m_hOwner: usize = 0x30; //
        pub const m_nProviderVisitedFlags: usize = 0x19F; //
        pub const m_bModifierStatesDirty: usize = 0x1A0; //
        pub const m_bPredictedOwner: usize = 0x1A1; //
        pub const m_iLockRefCount: usize = 0x1A2; //
        pub const m_hHandle: usize = 0x1A4; //
        pub const m_nBroadcastEventListenerMask: usize = 0x1A8; //
        pub const m_vecProviders: usize = 0x1B8; //
        pub const m_nDisabledGroups: usize = 0x1D0; //
        pub const m_bvEnabledStateMask: usize = 0x1D4; //
        pub const m_bvDisabledStateMask: usize = 0x1F0; //
        pub const m_bvEnabledPredictedStateMask: usize = 0x20C; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 2
    pub mod C_NPC_NanoRollermine {
        pub const m_flForwardSpeed: usize = 0x1438; //
        pub const m_hOwnerPawn: usize = 0x143C; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 1
    pub mod C_NPC_HeroCloneTrooper {
        pub const m_hOwner: usize = 0x1420; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_TenguUrn_Aura {
    }
    // Parent: CCitadelModifier
    // Field count: 5
    pub mod CGameModifier_OverrideTargetIdentifier {
        pub const m_sTargetIdentifier: usize = 0xC0; //
        pub const m_hTarget: usize = 0xC8; //
        pub const m_nOriginType: usize = 0xCC; //
        pub const m_sAttachmentName: usize = 0xD0; //
        pub const m_hAttachment: usize = 0xD8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_FealtyTarget {
    }
    // Parent: CitadelAbilityVData
    // Field count: 7
    pub mod CCitadel_Ability_RocketBarrageVData {
        pub const m_BarrageModifier: usize = 0x1590; //
        pub const m_MoveSlowModifier: usize = 0x15A0; //
        pub const m_ImpactParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExplodeSound: usize = 0x1690; //
        pub const m_cameraSequenceSelected: usize = 0x16A0; //
        pub const m_flMoveSpeedReductionPct: usize = 0x1728; //
        pub const m_flHeightTestDistance: usize = 0x172C; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_IncendiaryDebuff {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 44
    pub mod CCitadel_XPOrbVData {
        pub const m_bIsObjective: usize = 0x28; //
        pub const m_strOrbClaimed: usize = 0x30; //
        pub const m_strOrbClaimedTeammate: usize = 0x40; //
        pub const m_strOrbDenied: usize = 0x50; //
        pub const m_strOrbDeniedPlayer: usize = 0x60; //
        pub const m_strOrbHitConfirm: usize = 0x70; //
        pub const m_sOrbModel: usize = 0x80; //
        pub const m_sPredictedHitLimboGlowParticle: usize = 0x160; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sFriendlyHitConfirmParticle: usize = 0x240; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sEnemyHitConfirmParticle: usize = 0x320; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sFriendlyGlowParticle: usize = 0x400; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sEnemyGlowParticle: usize = 0x4E0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sGoldReceivedParticle: usize = 0x5C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sFriendlyOrbDeniedParticle: usize = 0x6A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sEnemyOrbDeniedParticle: usize = 0x780; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sFriendlyOrbEarnedParticle: usize = 0x860; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sEnemyOrbEarnedParticle: usize = 0x940; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flOrbSpawnDelayMin: usize = 0xA20; //
        pub const m_flOrbSpawnDelayMax: usize = 0xA24; //
        pub const m_flOrbSpawnOffsetZ: usize = 0xA28; //
        pub const m_flOrbSpawnOffsetRandomXYZ: usize = 0xA2C; //
        pub const m_flGravityScale: usize = 0xA30; //
        pub const m_flLateralSpeedMin: usize = 0xA34; //
        pub const m_flLateralSpeedMax: usize = 0xA38; //
        pub const m_flLateralMoveDuration: usize = 0xA3C; //
        pub const m_flUpSpeedMin: usize = 0xA40; //
        pub const m_flUpSpeedMax: usize = 0xA44; //
        pub const m_flBurstSpeedMultiplier: usize = 0xA48; //
        pub const m_flBurstSpeedDuration: usize = 0xA4C; //
        pub const m_flOscillateFrequency: usize = 0xA50; //
        pub const m_flLifeTime: usize = 0xA54; //
        pub const m_flRadius: usize = 0xA58; //
        pub const m_flCollisionRadius: usize = 0xA5C; //
        pub const m_flInvulDuration: usize = 0xA60; //
        pub const m_bUseKillerPlaneOffsets: usize = 0xA64; //
        pub const m_flKillerPlaneOffset: usize = 0xA68; //
        pub const m_flKillerPlaneHorizontalDecayRate: usize = 0xA6C; //
        pub const m_flKillerPlaneHorizontalSpeedX: usize = 0xA70; //
        pub const m_flKillerPlaneHorizontalSpeedY: usize = 0xA74; //
        pub const m_flKillerPlaneVerticalSpeed: usize = 0xA78; //
        pub const m_flKillerPlaneSpeedNoise: usize = 0xA7C; //
        pub const m_flKillerPlaneLaunchOffset: usize = 0xA80; //
        pub const m_flKillerPlaneLaunchDelay: usize = 0xA84; //
        pub const m_flOrbClaimWindow: usize = 0xA88; //
    }
    // Parent: None
    // Field count: 2
    pub mod EntityRenderAttribute_t {
        pub const m_ID: usize = 0x30; //
        pub const m_Values: usize = 0x34; //
    }
    // Parent: C_PhysicsProp
    // Field count: 1
    pub mod C_ShatterGlassShardPhysics {
        pub const m_ShardDesc: usize = 0xC90; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityTargetdummy3VData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Gunslinger_DemonMark {
        pub const m_flNextSearchTime: usize = 0xCA0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 7
    pub mod CCitadel_Ability_Tengu_StoneForm {
        pub const m_flStartTime: usize = 0xE68; //
        pub const m_flLandedTime: usize = 0xE6C; //
        pub const m_bLanded: usize = 0xE70; //
        pub const m_bFalling: usize = 0xE71; //
        pub const m_bInStoneForm: usize = 0xE72; //
        pub const m_flStartHeight: usize = 0xE74; //
        pub const m_nStoneFormEffect: usize = 0xE78; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_CheatDeathImmunity {
    }
    // Parent: C_BaseModelEntity
    // Field count: 1
    pub mod C_LightEntity {
        pub const m_CLightComponent: usize = 0x858; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 1
    pub mod CCitadel_Modifier_Cadence_Crescendo_AOE {
        pub const m_nTicks: usize = 0xF8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbility_Synth_Grasp_VData {
        pub const m_CasterModifier: usize = 0x1590; //
        pub const m_VictimModifier: usize = 0x15A0; //
        pub const m_BulletShieldModifier: usize = 0x15B0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Tokamak_DyingStar {
        pub const m_nRollFXIndex: usize = 0xCA0; //
        pub const m_bInFlight: usize = 0xCA4; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_SleepingVData {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_IceGrenade {
        pub const m_vLaunchPosition: usize = 0xCA0; //
        pub const m_qLaunchAngle: usize = 0xCAC; //
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_ChargePullEnemy {
        pub const m_vecOffsetDir: usize = 0xC0; //
        pub const m_flTackleRadius: usize = 0xCC; //
        pub const m_flPullTargetSpeed: usize = 0xD0; //
    }
    // Parent: CCitadel_WeaponUpgrade_HeadshotBooster_VData
    // Field count: 1
    pub mod CCitadel_WeaponUpgrade_Headhunter_VData {
        pub const m_HeadshotBuffModifier: usize = 0x15F8; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_SlowingTech_Proc {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Quarantine {
    }
    // Parent: CCitadel_Modifier_ShieldTracker_Base
    // Field count: 0
    pub mod CCitadel_Modifier_ShieldTracker_Bullet {
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_Base_Buildup {
        pub const m_flLastBuildupAppliedTime: usize = 0xC0; //
        pub const m_flDelayedDieTimeRemaining: usize = 0xC4; //
        pub const m_bInDelayTime: usize = 0xC8; //
        pub const m_flBuildUpDecayDelayFromWeaponCycleTime: usize = 0xCC; //
    }
    // Parent: C_BaseEntity
    // Field count: 6
    pub mod C_PlayerVisibility {
        pub const m_flVisibilityStrength: usize = 0x560; //
        pub const m_flFogDistanceMultiplier: usize = 0x564; //
        pub const m_flFogMaxDensityMultiplier: usize = 0x568; //
        pub const m_flFadeTime: usize = 0x56C; //
        pub const m_bStartDisabled: usize = 0x570; //
        pub const m_bIsEnabled: usize = 0x571; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_Magician_MagicBoltVData {
        pub const m_TargetDebuffModifier: usize = 0x1590; //
        pub const m_ExplodeParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_RetargetParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strRedirect: usize = 0x1760; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ReefdwellerHarpoon_DetachBuff {
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityRocketLauncherVData {
        pub const m_ExplosionParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Warden_CrowdControl_Debuff {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_FlameDash {
        pub const m_flDashEndTime: usize = 0xCA0; //
        pub const m_bIsSpeedBursting: usize = 0xCB8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_ArcaneEaterDebuffVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_Inhibitor_Proc {
    }
    // Parent: CitadelAbilityVData
    // Field count: 24
    pub mod CCitadel_Ability_Climb_RopeVData {
        pub const m_flMinButtonHoldTimeToActivate: usize = 0x1590; //
        pub const m_flClimbSpeedUp: usize = 0x1594; //
        pub const m_flClimbSpeedDown: usize = 0x1598; //
        pub const m_flClimbSpeedDownMax: usize = 0x159C; //
        pub const m_flClimbDownAccelTime: usize = 0x15A0; //
        pub const m_flLatchSpeed: usize = 0x15A4; //
        pub const m_flAttachOffset: usize = 0x15A8; //
        pub const m_flMinReconnectTime: usize = 0x15AC; //
        pub const m_flSideMoveReduction: usize = 0x15B0; //
        pub const m_flTopOffset: usize = 0x15B4; //
        pub const m_flBottomOffset: usize = 0x15B8; //
        pub const m_flTraceRadiusSize: usize = 0x15BC; //
        pub const m_flStopTimeToShoot: usize = 0x15C0; //
        pub const m_flJumpOffVertical: usize = 0x15C4; //
        pub const m_flJumpOffHorizontal: usize = 0x15C8; //
        pub const m_flDuckOffVertical: usize = 0x15CC; //
        pub const m_flDuckOffHorizontal: usize = 0x15D0; //
        pub const m_flActivateRange: usize = 0x15D4; //
        pub const m_flJumpToRoofRayCheckDist: usize = 0x15D8; //
        pub const m_flMinTimeToRoofCheck: usize = 0x15DC; //
        pub const m_flTimeToHintRefresh: usize = 0x15E0; //
        pub const m_iMaxHintCount: usize = 0x15E4; //
        pub const m_flClimbRopeSlowDurationOnHit: usize = 0x15E8; //
        pub const m_ClimbRopeSlowOnHitModifier: usize = 0x15F0; //
    }
    // Parent: None
    // Field count: 25
    pub mod fogparams_t {
        pub const dirPrimary: usize = 0x8; //
        pub const colorPrimary: usize = 0x14; //
        pub const colorSecondary: usize = 0x18; //
        pub const colorPrimaryLerpTo: usize = 0x1C; //
        pub const colorSecondaryLerpTo: usize = 0x20; //
        pub const start: usize = 0x24; //
        pub const end: usize = 0x28; //
        pub const farz: usize = 0x2C; //
        pub const maxdensity: usize = 0x30; //
        pub const exponent: usize = 0x34; //
        pub const HDRColorScale: usize = 0x38; //
        pub const skyboxFogFactor: usize = 0x3C; //
        pub const skyboxFogFactorLerpTo: usize = 0x40; //
        pub const startLerpTo: usize = 0x44; //
        pub const endLerpTo: usize = 0x48; //
        pub const maxdensityLerpTo: usize = 0x4C; //
        pub const lerptime: usize = 0x50; //
        pub const duration: usize = 0x54; //
        pub const blendtobackground: usize = 0x58; //
        pub const scattering: usize = 0x5C; //
        pub const locallightscale: usize = 0x60; //
        pub const enable: usize = 0x64; //
        pub const blend: usize = 0x65; //
        pub const m_bNoReflectionFog: usize = 0x66; //
        pub const m_bPadding: usize = 0x67; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 2
    pub mod C_Citadel_Projectile_Tier2Boss_RocketBarrage {
        pub const m_nLaserParticleIndex: usize = 0x8E0; //
        pub const m_vecSmoothedVelocity: usize = 0x8E4; //
    }
    // Parent: CCitadel_Modifier_Invis
    // Field count: 1
    pub mod CCitadel_Modifier_Shadow_Step {
        pub const m_nRevealedEffect: usize = 0x268; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CAbilityHatTrickVData {
        pub const m_SpectatingProjectileParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HatTrickChannelParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DebuffModifier: usize = 0x1830; //
        pub const m_strExplodeSound: usize = 0x1840; //
    }
    // Parent: CCitadelModifier
    // Field count: 7
    pub mod CCitadel_Modifier_VoidSphere {
        pub const m_bTeleported: usize = 0xC0; //
        pub const m_particleStart: usize = 0xC4; //
        pub const m_particleEnd: usize = 0xC8; //
        pub const m_particleTrail: usize = 0xCC; //
        pub const m_vecEndLocation: usize = 0xD0; //
        pub const m_vecStartPosition: usize = 0xDC; //
        pub const m_vecEndLocationCaster: usize = 0xE8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_ZiplineKnockdownImmuneVData {
        pub const m_ZipLineEnemyKnockdownProtectionParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineSelfKnockdownProtectionParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineKnockdownProtectionStatusParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ZipLineKnockdownProtectionStatusEnemyParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Upgrade_OverdriveClip_Reload {
        pub const m_nStartingClipSize: usize = 0xC0; //
    }
    // Parent: CitadelItemVData
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_HealOnLevelVData {
    }
    // Parent: CCitadelYamatoBaseVData
    // Field count: 29
    pub mod CCitadelAbilityFlyingStrikeVData {
        pub const m_flJumpFallSpeedMax: usize = 0x1598; //
        pub const m_flJumpAirDrag: usize = 0x159C; //
        pub const m_flJumpAirSpeedMax: usize = 0x15A0; //
        pub const m_flOnCancelVerticalSpeedBonus: usize = 0x15A4; //
        pub const m_flFlyingCloseEnoughToTarget: usize = 0x15A8; //
        pub const m_curveSpeedScale: usize = 0x15B0; //
        pub const m_flAnimToStrikePointTime: usize = 0x15F0; //
        pub const m_flAnimToStrikeArrivalBias: usize = 0x15F4; //
        pub const m_flGrappleShotFloatTime: usize = 0x15F8; //
        pub const m_flGrappleShotDelayToFlyOnHit: usize = 0x15FC; //
        pub const m_flGrappleSpeed: usize = 0x1600; //
        pub const m_SlowModifier: usize = 0x1608; //
        pub const m_GrappleTargetModifier: usize = 0x1618; //
        pub const m_WeaponBuffModifier: usize = 0x1628; //
        pub const m_LeapParticle: usize = 0x1638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x1718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SlashParticle: usize = 0x17F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BulletGrappleTracerParticle: usize = 0x18D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyGrappleParticle: usize = 0x19B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDamageTarget: usize = 0x1A98; //
        pub const m_strStartFlyingToTarget: usize = 0x1AA8; //
        pub const m_strStartAttack: usize = 0x1AB8; //
        pub const m_strGrappleHitTarget: usize = 0x1AC8; //
        pub const m_strGrappleHitWorld: usize = 0x1AD8; //
        pub const m_strGrappleHitNothing: usize = 0x1AE8; //
        pub const m_strGrappleLoop: usize = 0x1AF8; //
        pub const m_strFlyingLoop: usize = 0x1B08; //
        pub const m_cameraSequenceFlying: usize = 0x1B18; //
        pub const m_cameraSequenceAttacking: usize = 0x1BA0; //
    }
    // Parent: CCitadel_Modifier_Base
    // Field count: 0
    pub mod CCitadel_Modifier_FlyingStrikeTarget {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Nano_ShadowPulse {
    }
    // Parent: CitadelAbilityVData
    // Field count: 8
    pub mod CAbilityGooGrenadeVData {
        pub const m_GooGrenadeImpactModifier: usize = 0x1590; //
        pub const m_GooGrenadePuddleAuraModifier: usize = 0x15A0; //
        pub const m_GooGrenadeSkipParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_GooGrenadeExplodeParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_GrenadeHitImpactSound: usize = 0x1770; //
        pub const m_GrenadeMissImpactSound: usize = 0x1780; //
        pub const m_flMinRestitution: usize = 0x1790; //
        pub const m_flMaxRestitution: usize = 0x1794; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CCitadel_Ability_FireBombVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x1670; //
        pub const m_ProgressBarModifier: usize = 0x1680; //
        pub const m_FireBombModifier: usize = 0x1690; //
        pub const m_DebuffModifier: usize = 0x16A0; //
    }
    // Parent: CCitadel_Modifier_Tier3Boss_Base
    // Field count: 0
    pub mod CCitadel_Modifier_Tier3_DamagePulse {
    }
    // Parent: C_GameRulesProxy
    // Field count: 1
    pub mod C_CitadelGameRulesProxy {
        pub const m_pGameRules: usize = 0x560; //
    }
    // Parent: C_BaseEntity
    // Field count: 2
    pub mod CCitadelTrooperMinimap {
        pub const m_timeLastUpdate: usize = 0x560; //
        pub const m_vecFOWEntities: usize = 0x568; //
    }
    // Parent: C_NPC_SimpleAnimatingAI
    // Field count: 1
    pub mod C_NPC_BaseDefenseSentry {
        pub const m_vecUnitStatusOffset: usize = 0xB00; //
    }
    // Parent: C_DynamicProp
    // Field count: 5
    pub mod C_Citadel_DynamicProp {
        pub const m_nPlayerTeamEvent: usize = 0xDA8; //
        pub const m_strDefaultSkin: usize = 0xDB0; //
        pub const m_strFriendlySkin: usize = 0xDB8; //
        pub const m_strEnemySkin: usize = 0xDC0; //
        pub const m_bIsWorld: usize = 0xDC8; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 19
    pub mod C_BaseFlex {
        pub const m_flexWeight: usize = 0xB08; //
        pub const m_vLookTargetPosition: usize = 0xB20; //
        pub const m_blinktoggle: usize = 0xB38; //
        pub const m_nLastFlexUpdateFrameCount: usize = 0xB98; //
        pub const m_CachedViewTarget: usize = 0xB9C; //
        pub const m_nNextSceneEventId: usize = 0xBA8; //
        pub const m_iBlink: usize = 0xBAC; //
        pub const m_blinktime: usize = 0xBB0; //
        pub const m_prevblinktoggle: usize = 0xBB4; //
        pub const m_iJawOpen: usize = 0xBB8; //
        pub const m_flJawOpenAmount: usize = 0xBBC; //
        pub const m_flBlinkAmount: usize = 0xBC0; //
        pub const m_iMouthAttachment: usize = 0xBC4; //
        pub const m_iEyeAttachment: usize = 0xBC5; //
        pub const m_bResetFlexWeightsOnModelChange: usize = 0xBC6; //
        pub const m_nEyeOcclusionRendererBone: usize = 0xBE0; //
        pub const m_mEyeOcclusionRendererCameraToBoneTransform: usize = 0xBE4; //
        pub const m_vEyeOcclusionRendererHalfExtent: usize = 0xC14; //
        pub const m_PhonemeClasses: usize = 0xC30; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_DPS_Aura {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CGameModifier_FireConCommand {
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_MageWalkVData {
        pub const m_TeleportStartParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportEndParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportTrailParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flPreTeleportDuration: usize = 0x8D8; //
        pub const m_strAmbientLoopingLocalPlayerSound: usize = 0x8E0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CAbility_Fathom_LurkersAmbush_VData {
        pub const m_ChargeUpParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_InvisModifier: usize = 0x1670; //
        pub const m_RegenModifier: usize = 0x1680; //
        pub const m_ImmobilizeModifier: usize = 0x1690; //
        pub const m_strSwapStarted: usize = 0x16A0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 7
    pub mod CCitadel_Ability_Mirage_FireBeetles_VData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CasterModifier: usize = 0x1670; //
        pub const m_DebuffModifier: usize = 0x1680; //
        pub const m_StatStealModifier: usize = 0x1690; //
        pub const m_StealWatcherModifier: usize = 0x16A0; //
        pub const m_ExplosionSound: usize = 0x16B0; //
        pub const m_strHitConfirm: usize = 0x16C0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityTargetdummy4VData {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_SleepDagger_Drowsy_VData {
        pub const m_SleepModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 7
    pub mod CModifierIcePathVData {
        pub const m_FrontModel: usize = 0x638; //
        pub const m_BodyModel: usize = 0x718; //
        pub const m_GroundParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FloatingParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_IcePathBuffParticle: usize = 0x9B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FriendlyAuraModifier: usize = 0xA98; //
        pub const m_BonusSpiritLingerModifier: usize = 0xAA8; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_VitalitySuppressor {
        pub const m_flLastTickTime: usize = 0xC0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_SlowVData {
    }
    // Parent: C_BaseModelEntity
    // Field count: 12
    pub mod C_EnvSky {
        pub const m_hSkyMaterial: usize = 0x858; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_hSkyMaterialLightingOnly: usize = 0x860; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_bStartDisabled: usize = 0x868; //
        pub const m_vTintColor: usize = 0x869; //
        pub const m_vTintColorLightingOnly: usize = 0x86D; //
        pub const m_flBrightnessScale: usize = 0x874; //
        pub const m_nFogType: usize = 0x878; //
        pub const m_flFogMinStart: usize = 0x87C; //
        pub const m_flFogMinEnd: usize = 0x880; //
        pub const m_flFogMaxStart: usize = 0x884; //
        pub const m_flFogMaxEnd: usize = 0x888; //
        pub const m_bEnabled: usize = 0x88C; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 4
    pub mod CModifierGarbageAuraVData {
        pub const m_FinishParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AlliedParticle: usize = 0x760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyParticle: usize = 0x840; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAmbientLoopingLocalPlayerSound: usize = 0x920; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Item_Discord_Aura_Enemy {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ItemPickupPunchable {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Rutger_CheatDeath {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Tokamak_Breach {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 19
    pub mod CCitadel_Ability_Jump {
        pub const m_flLastTimeOnZipLine: usize = 0xCA0; //
        pub const m_flLastOnGroundTime: usize = 0xCA4; //
        pub const m_flPhaseStartTime: usize = 0xCA8; //
        pub const m_flJumpTime: usize = 0xCAC; //
        pub const m_LastJumpType: usize = 0xCB0; //
        pub const m_bShouldCreateAirJumpEffects: usize = 0xCB1; //
        pub const m_flDoubleJumpFailTime: usize = 0xCB4; //
        pub const m_eDoubleJumpFailReason: usize = 0xCB8; //
        pub const m_vWallJumpNormalUsed: usize = 0xCBC; //
        pub const m_flGroundDashJumpStartTime: usize = 0xDE0; //
        pub const m_flGroundDashJumpEndTime: usize = 0xDF8; //
        pub const m_bJumped: usize = 0xE10; //
        pub const m_bCanDashJump: usize = 0xE11; //
        pub const m_nDesiredAirJumpCount: usize = 0xE14; //
        pub const m_nExecutedAirJumpCount: usize = 0xE18; //
        pub const m_bInSlideJump: usize = 0xE1C; //
        pub const m_nConsecutiveAirJumps: usize = 0xE1D; //
        pub const m_nConsecutiveWallJumps: usize = 0xE1E; //
        pub const m_vLastWallCollidedWithNormal: usize = 0xE20; //
    }
    // Parent: C_PointEntity
    // Field count: 0
    pub mod CInfoTarget {
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_Nano_CatFormVData {
        pub const m_ModelChange: usize = 0x638; //
        pub const m_flModelScale: usize = 0x720; //
        pub const m_ExplodeSound: usize = 0x728; //
        pub const m_ImpactSound: usize = 0x738; //
    }
    // Parent: CCitadel_Modifier_Intrinsic_Base
    // Field count: 0
    pub mod CCitadel_Modifier_PredatorPrecision {
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_TechUpgrade_SuperAcolyteGlovesVData {
        pub const m_SpiritMeleeProcModifier: usize = 0x15D8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_TechCleaveVData {
        pub const m_CleavePlayerParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CleaveTrooperParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sVictimSound: usize = 0x7F8; //
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityPropertyMultiStats {
    }
    // Parent: C_BreakableProp
    // Field count: 23
    pub mod C_DynamicProp {
        pub const m_bUseHitboxesForRenderBox: usize = 0xC80; //
        pub const m_bUseAnimGraph: usize = 0xC81; //
        pub const m_pOutputAnimBegun: usize = 0xC88; //
        pub const m_pOutputAnimOver: usize = 0xCB0; //
        pub const m_pOutputAnimLoopCycleOver: usize = 0xCD8; //
        pub const m_OnAnimReachedStart: usize = 0xD00; //
        pub const m_OnAnimReachedEnd: usize = 0xD28; //
        pub const m_iszIdleAnim: usize = 0xD50; //
        pub const m_nIdleAnimLoopMode: usize = 0xD58; //
        pub const m_bRandomizeCycle: usize = 0xD5C; //
        pub const m_bStartDisabled: usize = 0xD5D; //
        pub const m_bFiredStartEndOutput: usize = 0xD5E; //
        pub const m_bForceNpcExclude: usize = 0xD5F; //
        pub const m_bCreateNonSolid: usize = 0xD60; //
        pub const m_bIsOverrideProp: usize = 0xD61; //
        pub const m_iInitialGlowState: usize = 0xD64; //
        pub const m_nGlowRange: usize = 0xD68; //
        pub const m_nGlowRangeMin: usize = 0xD6C; //
        pub const m_glowColor: usize = 0xD70; //
        pub const m_nGlowTeam: usize = 0xD74; //
        pub const m_iCachedFrameCount: usize = 0xD78; //
        pub const m_vecCachedRenderMins: usize = 0xD7C; //
        pub const m_vecCachedRenderMaxs: usize = 0xD88; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_WeaponUpgrade_SpellslingerHeadshots {
    }
    // Parent: CCitadel_Item
    // Field count: 1
    pub mod CCitadel_ArmorUpgrade_SpellShield {
        pub const fl_mSpellShieldBreakTime: usize = 0xCB8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CCitadel_Ability_TurretClone_VData {
        pub const m_strTurretParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strSwapParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TurretSound: usize = 0x1750; //
        pub const m_SwapSound: usize = 0x1760; //
        pub const m_BuffModifier: usize = 0x1770; //
        pub const m_cameraSequenceTeleport: usize = 0x1780; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_VandalOverflow {
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CAbilityChargedTackleVData {
        pub const m_ChargePreviewParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChargePrepareModifier: usize = 0x1670; //
        pub const m_ChargeActiveModifier: usize = 0x1680; //
        pub const m_DragModifier: usize = 0x1690; //
        pub const m_strHitSound: usize = 0x16A0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_SpilledBloodThinkerVData {
        pub const m_SpilledBloodParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flTickRate: usize = 0x718; //
        pub const m_flHeight: usize = 0x71C; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Fervor {
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_AttachTarget {
        pub const m_hTarget: usize = 0xC0; //
        pub const m_vecOffset: usize = 0xC4; //
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 6
    pub mod CDestructableBuildingVData {
        pub const m_iMaxHealthFinal: usize = 0x28; //
        pub const m_iMaxHealthGenerator: usize = 0x2C; //
        pub const m_ObjectiveRegen: usize = 0x30; //
        pub const m_BackdoorBulletResistModifier: usize = 0x40; //
        pub const m_BackdoorProtectionModifier: usize = 0x50; //
        pub const m_RangedArmorModifier: usize = 0x60; //
    }
    // Parent: CEntityComponent
    // Field count: 10
    pub mod CPropDataComponent {
        pub const m_flDmgModBullet: usize = 0x10; //
        pub const m_flDmgModClub: usize = 0x14; //
        pub const m_flDmgModExplosive: usize = 0x18; //
        pub const m_flDmgModFire: usize = 0x1C; //
        pub const m_iszPhysicsDamageTableName: usize = 0x20; //
        pub const m_iszBasePropData: usize = 0x28; //
        pub const m_nInteractions: usize = 0x30; //
        pub const m_bSpawnMotionDisabled: usize = 0x34; //
        pub const m_nDisableTakePhysicsDamageSpawnFlag: usize = 0x38; //
        pub const m_nMotionDisabledSpawnFlag: usize = 0x3C; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 2
    pub mod C_NPC_TrooperNeutral {
        pub const m_bPlayingIdle: usize = 0x1420; //
        pub const m_bShieldActive: usize = 0x1421; //
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 1
    pub mod CModifierGravityLassoEnemyVData {
        pub const m_LassoEffect: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Wraith_RapidFireVData {
        pub const m_CastParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetBuffSound: usize = 0x1670; //
        pub const m_RapidFireModifier: usize = 0x1680; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 1
    pub mod C_ItemWeaponParts {
        pub const m_hTouchedPlayeres: usize = 0xB18; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbility_Operative_UmbrellaManeuver_VData {
        pub const m_WatcherModifier: usize = 0x1590; //
        pub const m_LaunchParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbilityStackingDamageVData {
        pub const m_StackingModifier: usize = 0x1590; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 2
    pub mod CCitadel_Modifier_TechDamageProcWatcher {
        pub const m_flNextProcTime: usize = 0x168; //
        pub const m_shotProced: usize = 0x16C; //
    }
    // Parent: C_NPC_Boss_Tier2
    // Field count: 0
    pub mod C_NPC_Boss_Tier2_Sidelanes {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Citadel_Projectile_BloodBomb {
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 3
    pub mod CCitadel_Item_Discord_AuraVData_Enemy {
        pub const m_strAreaEffectEnemy: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAreaEffectFriendly: usize = 0x760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAreaEffectSelf: usize = 0x840; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_BaseEntity
    // Field count: 8
    pub mod CCitadelSoundOpvarSetOBB {
        pub const m_iszStackName: usize = 0x578; //
        pub const m_iszOperatorName: usize = 0x580; //
        pub const m_iszOpvarName: usize = 0x588; //
        pub const m_vDistanceInnerMins: usize = 0x590; //
        pub const m_vDistanceInnerMaxs: usize = 0x59C; //
        pub const m_vDistanceOuterMins: usize = 0x5A8; //
        pub const m_vDistanceOuterMaxs: usize = 0x5B4; //
        pub const m_nAABBDirection: usize = 0x5C0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 10
    pub mod CCitadel_Ability_Nano_Pounce {
        pub const m_bActive: usize = 0xED0; //
        pub const m_hCurrentTarget: usize = 0xED4; //
        pub const m_hLastCastTarget: usize = 0xED8; //
        pub const m_vStartPosition: usize = 0xEDC; //
        pub const m_vDeparturePosition: usize = 0xEE8; //
        pub const m_flDepartureTime: usize = 0xEF8; //
        pub const m_flArrivalTime: usize = 0xF10; //
        pub const m_vLastKnownSafePos: usize = 0xF28; //
        pub const m_bIsFirstCastCompleted: usize = 0xF36; //
        pub const m_tDoubleCastWindow: usize = 0xF38; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_AOERoot {
    }
    // Parent: CNPC_TrooperBossVData
    // Field count: 4
    pub mod CNPC_TrooperBarrackBossVData {
        pub const m_flBackDoorProtectionRange: usize = 0x1808; //
        pub const m_BackdoorProtectionModifier: usize = 0x1810; //
        pub const m_BackdoorBulletResistModifier: usize = 0x1820; //
        pub const m_ObjectiveRegen: usize = 0x1830; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_Wrecker_UltimateGrabEnemyVData {
        pub const m_EnemyHeroStasisEffect: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_EnemyHeroGrabEffect: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod CFilterModel {
        pub const m_iFilterModel: usize = 0x5B8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Synth_Barrage_Amp {
    }
    // Parent: CitadelAbilityVData
    // Field count: 12
    pub mod CAbilityHornetLeapVData {
        pub const m_flChannelingAirDrag: usize = 0x1590; //
        pub const m_flChannelingMaxFallSpeed: usize = 0x1594; //
        pub const m_flVerticalMoveSpeedPercent: usize = 0x1598; //
        pub const m_flAirDrag: usize = 0x159C; //
        pub const m_flAirAcceleration: usize = 0x15A0; //
        pub const m_flLaunchAirDrag: usize = 0x15A4; //
        pub const m_flLaunchTime: usize = 0x15A8; //
        pub const m_flMoveSpeedAboveBaseScale: usize = 0x15AC; //
        pub const m_LeapModifier: usize = 0x15B0; //
        pub const m_DustParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TrailParticle: usize = 0x16A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1780; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_Item_AOE_Tech_ShieldVData {
        pub const m_DurationModifier: usize = 0x15D8; //
    }
    // Parent: C_GameRules
    // Field count: 0
    pub mod C_SingleplayRules {
    }
    // Parent: CBaseAnimGraph
    // Field count: 25
    pub mod C_LocalTempEntity {
        pub const flags: usize = 0xAF8; //
        pub const die: usize = 0xAFC; //
        pub const m_flFrameMax: usize = 0xB00; //
        pub const x: usize = 0xB04; //
        pub const y: usize = 0xB08; //
        pub const fadeSpeed: usize = 0xB0C; //
        pub const bounceFactor: usize = 0xB10; //
        pub const hitSound: usize = 0xB14; //
        pub const priority: usize = 0xB18; //
        pub const tentOffset: usize = 0xB1C; //
        pub const m_vecTempEntAngVelocity: usize = 0xB28; //
        pub const tempent_renderamt: usize = 0xB34; //
        pub const m_vecNormal: usize = 0xB38; //
        pub const m_flSpriteScale: usize = 0xB44; //
        pub const m_nFlickerFrame: usize = 0xB48; //
        pub const m_flFrameRate: usize = 0xB4C; //
        pub const m_flFrame: usize = 0xB50; //
        pub const m_pszImpactEffect: usize = 0xB58; //
        pub const m_pszParticleEffect: usize = 0xB60; //
        pub const m_bParticleCollision: usize = 0xB68; //
        pub const m_iLastCollisionFrame: usize = 0xB6C; //
        pub const m_vLastCollisionOrigin: usize = 0xB70; //
        pub const m_vecTempEntVelocity: usize = 0xB7C; //
        pub const m_vecPrevAbsOrigin: usize = 0xB88; //
        pub const m_vecTempEntAcceleration: usize = 0xB94; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Thumper_Bullet_Watcher {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Protection_Racket {
    }
    // Parent: CCitadelBaseYamatoAbility
    // Field count: 2
    pub mod CCitadel_Ability_InfinitySlash {
        pub const m_flExplodeEndTime: usize = 0xDC0; //
        pub const m_flBuffEndTime: usize = 0xDC4; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadelModifierProjectilePitchingLoopSoundThinker {
    }
    // Parent: CEntitySubclassVDataBase
    // Field count: 8
    pub mod CCitadel_HeroTestOrbSpawnerVData {
        pub const m_iGoldValue: usize = 0x28; //
        pub const m_flSpawnRate: usize = 0x2C; //
        pub const m_flFirstSpawnTime: usize = 0x30; //
        pub const m_hModel: usize = 0x38; //
        pub const m_flModelScale: usize = 0x118; //
        pub const m_flSpawnOffset: usize = 0x11C; //
        pub const m_AmbientParticle: usize = 0x120; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SpawnParticle: usize = 0x200; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: None
    // Field count: 2
    pub mod C_EnvWindShared__WindAveEvent_t {
        pub const m_flStartWindSpeed: usize = 0x0; //
        pub const m_flAveWindSpeed: usize = 0x4; //
    }
    // Parent: C_LightDirectionalEntity
    // Field count: 0
    pub mod C_LightEnvironmentEntity {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Tokamak_EnemySmokeAOE {
    }
    // Parent: CCitadelPlayerController
    // Field count: 0
    pub mod CCitadelPreviewPlayerController {
    }
    // Parent: CAI_CitadelNPCVData
    // Field count: 45
    pub mod CNPC_Boss_Tier3VData {
        pub const m_nPhase2Health: usize = 0x10E8; //
        pub const m_flEyeZOffset: usize = 0x10EC; //
        pub const m_flDefaultMoveSpeed: usize = 0x10F0; //
        pub const m_flNoShieldMoveSpeed: usize = 0x10F4; //
        pub const m_flDyingMoveSpeed: usize = 0x10F8; //
        pub const m_flMovingToFinalPositionSpeed: usize = 0x10FC; //
        pub const m_DeathSmallExplosionParticle: usize = 0x1100; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DeathLargeExplosionParticle: usize = 0x11E0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WeakpointBrokenExplosionParticle: usize = 0x12C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChargeUpExplosionParticle: usize = 0x13A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strWIPModelName: usize = 0x1480; //
        pub const m_DyingSmallExplosion: usize = 0x1560; //
        pub const m_AvatarKilledSound: usize = 0x1570; //
        pub const m_AvatarBecomePatronSound: usize = 0x1580; //
        pub const m_PatronLandedSound: usize = 0x1590; //
        pub const m_PatronKilledSound: usize = 0x15A0; //
        pub const m_LaserSound: usize = 0x15B0; //
        pub const m_LaserBeamModifier: usize = 0x15C0; //
        pub const m_DyingModifier: usize = 0x15D0; //
        pub const m_VulnerableModifier: usize = 0x15E0; //
        pub const m_Phase1Modifier: usize = 0x15F0; //
        pub const m_Phase2Modifier: usize = 0x1600; //
        pub const m_BackdoorProtection: usize = 0x1610; //
        pub const m_RangedArmorModifier: usize = 0x1620; //
        pub const m_ObjectiveRegen: usize = 0x1630; //
        pub const m_ObjectiveHealthGrowthPhase1: usize = 0x1640; //
        pub const m_ObjectiveHealthGrowthPhase2: usize = 0x1650; //
        pub const m_LaserChargingParticle: usize = 0x1660; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LaserBeamEffect: usize = 0x1740; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LaserPreviewEffect: usize = 0x1820; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LaserDamageEffect: usize = 0x1900; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flLaserTargetingZOffset: usize = 0x19E0; //
        pub const m_flLaserTrackingSpeed: usize = 0x19E4; //
        pub const m_flLaserTrackingMaxSpeed: usize = 0x19E8; //
        pub const m_flLaserCastingTrackSpeed: usize = 0x19EC; //
        pub const m_flLaserCastingTrackMaxSpeed: usize = 0x19F0; //
        pub const m_flLaserDPSToPlayers: usize = 0x19F4; //
        pub const m_flLaserDPSToNPCs: usize = 0x19F8; //
        pub const m_flLaserDPSMaxHealth: usize = 0x19FC; //
        pub const m_flNoShieldLaserTrackingSpeed: usize = 0x1A00; //
        pub const m_flNoShieldLaserTrackingMaxSpeed: usize = 0x1A04; //
        pub const m_flNoShieldLaserCastingTrackSpeed: usize = 0x1A08; //
        pub const m_flNoShieldLaserCastingTrackMaxSpeed: usize = 0x1A0C; //
        pub const m_flNoShieldLaserDPSToPlayers: usize = 0x1A10; //
        pub const m_flNoShieldLaserDPSToNPCs: usize = 0x1A14; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Hornet_Chain {
        pub const m_vLaunchPosition: usize = 0xCA0; //
        pub const m_qLaunchAngle: usize = 0xCAC; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CItemSingleTargetStunVData {
        pub const m_StunDelayModifier: usize = 0x15D8; //
        pub const m_CastParticle: usize = 0x15E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Item_TrackingProjectileApplyModifierVData
    // Field count: 1
    pub mod CItem_WitheringWhip_VData {
        pub const m_DebuffModifier: usize = 0x16D8; //
    }
    // Parent: C_PointEntity
    // Field count: 9
    pub mod CInfoOffscreenPanoramaTexture {
        pub const m_bDisabled: usize = 0x560; //
        pub const m_nResolutionX: usize = 0x564; //
        pub const m_nResolutionY: usize = 0x568; //
        pub const m_szLayoutFileName: usize = 0x570; //
        pub const m_RenderAttrName: usize = 0x578; //
        pub const m_TargetEntities: usize = 0x580; //
        pub const m_nTargetChangeCount: usize = 0x598; //
        pub const m_vecCSSClasses: usize = 0x5A0; // C_NetworkUtlVectorBase<CUtlSymbolLarge>
        pub const m_bCheckCSSClasses: usize = 0x718; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 7
    pub mod C_DynamicLight {
        pub const m_Flags: usize = 0x858; //
        pub const m_LightStyle: usize = 0x859; //
        pub const m_Radius: usize = 0x85C; //
        pub const m_Exponent: usize = 0x860; //
        pub const m_InnerAngle: usize = 0x864; //
        pub const m_OuterAngle: usize = 0x868; //
        pub const m_SpotRadius: usize = 0x86C; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CAbility_Operative_Revelation_VData {
        pub const m_CasterModifier: usize = 0x1590; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_ViperHookblade {
        pub const m_vecOutgoingHitList: usize = 0xCA0; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_vecReturningHitList: usize = 0xCB8; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_ShieldGuy_Ability04 {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_Nano_ClusterGrenadeVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AuraModifier: usize = 0x1670; //
        pub const m_ExplodeSound: usize = 0x1680; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Nano_Shadow {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Bull_Leap_BoostingVData {
        pub const m_BoostTrailParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_ArmorUpgrade_CloakingDeviceActive_VData {
        pub const m_AmbushModifier: usize = 0x15D8; //
        pub const m_InvisModifier: usize = 0x15E8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TeamRelativeParticle {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Rutger_CheatDeath_Activated {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Warden_HighAlert {
    }
    // Parent: CitadelAbilityVData
    // Field count: 24
    pub mod CAbilityGuidedArrowVData {
        pub const m_cameraCancelledTransitionBacktoArcher: usize = 0x1590; //
        pub const m_cameraExplodedTransitionBackToArcher: usize = 0x1618; //
        pub const m_flCameraHoldAtExplosion: usize = 0x16A0; //
        pub const m_flFadeToBlackTime: usize = 0x16A4; //
        pub const m_SpectatingProjectileParticle: usize = 0x16A8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionParticle: usize = 0x1788; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_GuidedArrowChannelParticle: usize = 0x1868; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ProjectileModel: usize = 0x1948; //
        pub const m_ArrowOffsetX: usize = 0x1A28; //
        pub const m_ArrowCameraDistance: usize = 0x1A2C; //
        pub const m_ArrowCameraHeightOffset: usize = 0x1A30; //
        pub const m_ArrowInitialPitch: usize = 0x1A34; //
        pub const m_GuidingModifier: usize = 0x1A38; //
        pub const m_DebuffModifier: usize = 0x1A48; //
        pub const m_KillCheckModifier: usize = 0x1A58; //
        pub const m_strExplodeSound: usize = 0x1A68; //
        pub const m_flTrackAmount: usize = 0x1A78; //
        pub const m_flSpeedAccel: usize = 0x1A7C; //
        pub const m_flSpeedDeccel: usize = 0x1A80; //
        pub const m_flBaseProjectileSpeed: usize = 0x1A84; //
        pub const m_flMaxProjectileSpeed: usize = 0x1A88; //
        pub const m_flArrowModelTurnSpringStrength: usize = 0x1A8C; //
        pub const m_flKillCheckWindow: usize = 0x1A90; //
        pub const m_flWorldCollideGraceWindow: usize = 0x1A94; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityPowerJumpVData {
        pub const m_JumpParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_InAirModifier: usize = 0x1670; //
        pub const m_PowerJumpModifier: usize = 0x1680; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_NearbyEnemyResist {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_PersonalRejuvenator {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_PayloadPusherAuraTarget {
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_TriggerVolume {
    }
    // Parent: C_FuncBrush
    // Field count: 3
    pub mod C_FuncElectrifiedVolume {
        pub const m_nAmbientEffect: usize = 0x858; //
        pub const m_EffectName: usize = 0x860; //
        pub const m_bState: usize = 0x868; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Tokamak_Radiance {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadelModifierCadenceGunSpikesVData {
        pub const m_strSmallIconCssClassMax: usize = 0x638; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_Cadence_GrandFinale_BuffVData {
        pub const m_BuildUpModifier: usize = 0x638; //
        pub const m_ExplodeParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeSound: usize = 0x728; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_AnthemBuffVData {
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadelYamatoBaseVData {
        pub const m_flShadowFormSpeed: usize = 0x1590; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_TangoTether_TetherReceiverVData {
        pub const m_strAttackBuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TetherSound: usize = 0x718; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_WreckerSalvageVData {
        pub const m_SalvageBeam: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ConnectBeam: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_HatTrick {
        pub const m_hProjectile: usize = 0xCA0; //
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 1
    pub mod CCitadel_Modifier_Ricochet_ProcVData {
        pub const m_RicochetTracerParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelBaseAbilityServerOnly
    // Field count: 0
    pub mod CCitadel_Ability_Weapon_BossTier2 {
    }
    // Parent: CCitadelModifier
    // Field count: 7
    pub mod CCitadel_Modifier_Invis {
        pub const m_bInvis: usize = 0x248; //
        pub const m_flStartInvisTime: usize = 0x24C; //
        pub const m_bFullyInvis: usize = 0x250; //
        pub const m_flLastDamageTaken: usize = 0x254; //
        pub const m_flLastSpotted: usize = 0x258; //
        pub const m_nDetectionRangeRing: usize = 0x25C; //
        pub const m_nFullInvisEffect: usize = 0x260; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CModifier_Mirage_SandPhantom_VData {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Mirage_Tornado_Caster {
    }
    // Parent: CCitadel_Modifier_Sleep
    // Field count: 0
    pub mod CCitadel_Modifier_SleepBomb_Asleep {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Lockdown_BulletResist {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_WreckerUltimate_Invincible {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Lash {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CCitadel_Ability_BloodBombVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SpilledBloodModifier: usize = 0x1670; //
        pub const m_strBloodSpillStatName: usize = 0x1680; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_LongRangeSlowingTech_Proc {
    }
    // Parent: C_BaseEntity
    // Field count: 2
    pub mod C_EnvDetailController {
        pub const m_flFadeStartDist: usize = 0x560; //
        pub const m_flFadeEndDist: usize = 0x564; //
    }
    // Parent: CEntityInstance
    // Field count: 81
    pub mod C_BaseEntity {
        pub const m_CBodyComponent: usize = 0x38; //
        pub const m_NetworkTransmitComponent: usize = 0x40; //
        pub const m_nLastThinkTick: usize = 0x328; //
        pub const m_pGameSceneNode: usize = 0x330; //
        pub const m_pRenderComponent: usize = 0x338; //
        pub const m_pCollision: usize = 0x340; //
        pub const m_pModifierProp: usize = 0x348; //
        pub const m_iMaxHealth: usize = 0x350; //
        pub const m_iHealth: usize = 0x354; //
        pub const m_lifeState: usize = 0x358; //
        pub const m_bTakesDamage: usize = 0x359; //
        pub const m_nTakeDamageFlags: usize = 0x360; //
        pub const m_nPlatformType: usize = 0x368; //
        pub const m_ubInterpolationFrame: usize = 0x369; //
        pub const m_hSceneObjectController: usize = 0x36C; //
        pub const m_nNoInterpolationTick: usize = 0x370; //
        pub const m_nVisibilityNoInterpolationTick: usize = 0x374; //
        pub const m_flProxyRandomValue: usize = 0x378; //
        pub const m_iEFlags: usize = 0x37C; //
        pub const m_nWaterType: usize = 0x380; //
        pub const m_bInterpolateEvenWithNoModel: usize = 0x381; //
        pub const m_bPredictionEligible: usize = 0x382; //
        pub const m_bApplyLayerMatchIDToModel: usize = 0x383; //
        pub const m_tokLayerMatchID: usize = 0x384; //
        pub const m_nSubclassID: usize = 0x388; //
        pub const m_nSimulationTick: usize = 0x398; //
        pub const m_iCurrentThinkContext: usize = 0x39C; //
        pub const m_aThinkFunctions: usize = 0x3A0; //
        pub const m_bDisabledContextThinks: usize = 0x3B8; //
        pub const m_flAnimTime: usize = 0x3BC; //
        pub const m_flSimulationTime: usize = 0x3C0; //
        pub const m_nSceneObjectOverrideFlags: usize = 0x3C4; //
        pub const m_bHasSuccessfullyInterpolated: usize = 0x3C5; //
        pub const m_bHasAddedVarsToInterpolation: usize = 0x3C6; //
        pub const m_bRenderEvenWhenNotSuccessfullyInterpolated: usize = 0x3C7; //
        pub const m_nInterpolationLatchDirtyFlags: usize = 0x3C8; //
        pub const m_ListEntry: usize = 0x3D0; //
        pub const m_flCreateTime: usize = 0x3E8; //
        pub const m_flSpeed: usize = 0x3EC; //
        pub const m_EntClientFlags: usize = 0x3F0; //
        pub const m_bClientSideRagdoll: usize = 0x3F2; //
        pub const m_iTeamNum: usize = 0x3F3; //
        pub const m_spawnflags: usize = 0x3F4; //
        pub const m_nNextThinkTick: usize = 0x3F8; //
        pub const m_fFlags: usize = 0x400; //
        pub const m_vecAbsVelocity: usize = 0x404; //
        pub const m_vecVelocity: usize = 0x410; //
        pub const m_hEffectEntity: usize = 0x440; //
        pub const m_hOwnerEntity: usize = 0x444; //
        pub const m_MoveCollide: usize = 0x448; //
        pub const m_MoveType: usize = 0x449; //
        pub const m_nActualMoveType: usize = 0x44A; //
        pub const m_flWaterLevel: usize = 0x44C; //
        pub const m_fEffects: usize = 0x450; //
        pub const m_hGroundEntity: usize = 0x454; //
        pub const m_nGroundBodyIndex: usize = 0x458; //
        pub const m_flFriction: usize = 0x45C; //
        pub const m_flElasticity: usize = 0x460; //
        pub const m_flGravityScale: usize = 0x464; //
        pub const m_flTimeScale: usize = 0x468; //
        pub const m_bAnimatedEveryTick: usize = 0x46C; //
        pub const m_flNavIgnoreUntilTime: usize = 0x470; //
        pub const m_hThink: usize = 0x474; //
        pub const m_fBBoxVisFlags: usize = 0x480; //
        pub const m_bPredictable: usize = 0x481; //
        pub const m_bRenderWithViewModels: usize = 0x482; //
        pub const m_nSplitUserPlayerPredictionSlot: usize = 0x484; //
        pub const m_nFirstPredictableCommand: usize = 0x488; //
        pub const m_nLastPredictableCommand: usize = 0x48C; //
        pub const m_hOldMoveParent: usize = 0x490; //
        pub const m_Particles: usize = 0x498; //
        pub const m_vecPredictedScriptFloats: usize = 0x4C0; //
        pub const m_vecPredictedScriptFloatIDs: usize = 0x4D8; //
        pub const m_nNextScriptVarRecordID: usize = 0x508; //
        pub const m_vecAngVelocity: usize = 0x518; //
        pub const m_DataChangeEventRef: usize = 0x524; //
        pub const m_dependencies: usize = 0x528; //
        pub const m_nCreationTick: usize = 0x540; //
        pub const m_bAnimTimeChanged: usize = 0x54D; //
        pub const m_bSimulationTimeChanged: usize = 0x54E; //
        pub const m_sUniqueHammerID: usize = 0x558; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CGameModifier_FireConCommandVData {
        pub const m_FireOnAdded: usize = 0x638; //
        pub const m_FireOnRemoved: usize = 0x640; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 7
    pub mod CModifier_Mirage_SandPhantom_WhirlwindEvasion_VData {
        pub const m_AttackerHitFx: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_playerBuffSelf: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_playerBuffEnemy: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ReflectedBulletTracerParticle: usize = 0x9B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAttackerHitSound: usize = 0xA98; //
        pub const m_strVictimHitSound: usize = 0xAA8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_SilenceContraptionsDebuff {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Spinning_Blade {
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CModifierAirLiftExplodeTargetVData {
        pub const m_strSilenceTargetSound: usize = 0x638; //
        pub const m_SilenceModifier: usize = 0x648; //
        pub const m_SlowModifier: usize = 0x658; //
        pub const m_BulletResistModifier: usize = 0x668; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_DeathTax {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_StaticCharge {
    }
    // Parent: CitadelItemVData
    // Field count: 3
    pub mod CCitadel_ArmorUpgrade_DebuffReducerVData {
        pub const m_DebuffReducedParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PurgeCastParticle: usize = 0x16B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_MoveSpeedModifier: usize = 0x1798; //
    }
    // Parent: CScaleFunctionBase
    // Field count: 0
    pub mod CScaleFunctionAbilityProperty_TechDamage {
    }
    // Parent: None
    // Field count: 11
    pub mod CEntityIdentity {
        pub const m_nameStringableIndex: usize = 0x14; //
        pub const m_name: usize = 0x18; //
        pub const m_designerName: usize = 0x20; //
        pub const m_flags: usize = 0x30; //
        pub const m_worldGroupId: usize = 0x38; //
        pub const m_fDataObjectTypes: usize = 0x3C; //
        pub const m_PathIndex: usize = 0x40; //
        pub const m_pPrev: usize = 0x58; //
        pub const m_pNext: usize = 0x60; //
        pub const m_pPrevByClass: usize = 0x68; //
        pub const m_pNextByClass: usize = 0x70; //
    }
    // Parent: CCitadelAnimatingModelEntity
    // Field count: 6
    pub mod C_Citadel_Bounce_Pad {
        pub const m_flUpFactor: usize = 0xB00; //
        pub const m_flBounceVelocity: usize = 0xB04; //
        pub const m_flBarrelBounceVelocity: usize = 0xB08; //
        pub const m_flBarrelUpFactor: usize = 0xB0C; //
        pub const m_bSpeedOnLand: usize = 0xB10; //
        pub const m_bHasBouncedPlayerBefore: usize = 0xB11; //
    }
    // Parent: C_BreakableProp
    // Field count: 1
    pub mod C_PhysicsProp {
        pub const m_bAwake: usize = 0xC80; //
    }
    // Parent: CBaseProp
    // Field count: 29
    pub mod C_BreakableProp {
        pub const m_CPropDataComponent: usize = 0xB38; //
        pub const m_OnBreak: usize = 0xB78; //
        pub const m_OnHealthChanged: usize = 0xBA0; // CEntityOutputTemplate<float32>
        pub const m_OnTakeDamage: usize = 0xBC8; //
        pub const m_impactEnergyScale: usize = 0xBF0; //
        pub const m_iMinHealthDmg: usize = 0xBF4; //
        pub const m_flPressureDelay: usize = 0xBF8; //
        pub const m_flDefBurstScale: usize = 0xBFC; //
        pub const m_vDefBurstOffset: usize = 0xC00; //
        pub const m_hBreaker: usize = 0xC0C; //
        pub const m_PerformanceMode: usize = 0xC10; //
        pub const m_flPreventDamageBeforeTime: usize = 0xC14; //
        pub const m_BreakableContentsType: usize = 0xC18; //
        pub const m_strBreakableContentsPropGroupOverride: usize = 0xC20; //
        pub const m_strBreakableContentsParticleOverride: usize = 0xC28; //
        pub const m_bHasBreakPiecesOrCommands: usize = 0xC30; //
        pub const m_explodeDamage: usize = 0xC34; //
        pub const m_explodeRadius: usize = 0xC38; //
        pub const m_explosionDelay: usize = 0xC40; //
        pub const m_explosionBuildupSound: usize = 0xC48; //
        pub const m_explosionCustomEffect: usize = 0xC50; //
        pub const m_explosionCustomSound: usize = 0xC58; //
        pub const m_explosionModifier: usize = 0xC60; //
        pub const m_hPhysicsAttacker: usize = 0xC68; //
        pub const m_flLastPhysicsInfluenceTime: usize = 0xC6C; //
        pub const m_flDefaultFadeScale: usize = 0xC70; //
        pub const m_hLastAttacker: usize = 0xC74; //
        pub const m_hFlareEnt: usize = 0xC78; //
        pub const m_noGhostCollision: usize = 0xC7C; //
    }
    // Parent: CCitadelBaseLockonAbility
    // Field count: 4
    pub mod CCitadel_Ability_Lash_Ultimate {
        pub const m_EGrappleState: usize = 0xE32; //
        pub const m_flStateEnterTime: usize = 0xE34; //
        pub const m_flNextStateTime: usize = 0xE38; //
        pub const m_flBoostEndTime: usize = 0xE3C; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadelModifierItemPickupTimerVData {
        pub const m_TimerToSilence: usize = 0x638; //
        pub const m_SilenceDuration: usize = 0x63C; //
        pub const m_SilenceModifier: usize = 0x640; //
        pub const m_bIsIdolPickup: usize = 0x650; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_Rutger_Pulse_VData {
        pub const m_AuraModifier: usize = 0x1590; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 0
    pub mod CAbilityThumper1VData {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Uppercut_Buff {
    }
    // Parent: CitadelAbilityVData
    // Field count: 37
    pub mod CAbilityViscousBowlingVData {
        pub const m_TransformStartFx: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeFX: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_WallImpactFx: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BallTrailFx: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_GroundImpactParticle: usize = 0x1910; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_JumpParticle: usize = 0x19F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DirectionParticle: usize = 0x1AD0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flVerticalCameraOffsetLerpTime: usize = 0x1BB0; //
        pub const m_flVerticalCameraOffsetBias: usize = 0x1BB4; //
        pub const m_flVerticalCameraOffset: usize = 0x1BB8; //
        pub const m_flDistanceCameraOffsetLerpTime: usize = 0x1BBC; //
        pub const m_flDistanceCameraOffsetBias: usize = 0x1BC0; //
        pub const m_flDistanceCameraOffset: usize = 0x1BC4; //
        pub const m_strPopGraphParamter: usize = 0x1BC8; //
        pub const m_BallJumpSound: usize = 0x1BD0; //
        pub const m_EnterBallSound: usize = 0x1BE0; //
        pub const m_BallLoopSound: usize = 0x1BF0; //
        pub const m_ExitBallSound: usize = 0x1C00; //
        pub const m_WallImpactSound: usize = 0x1C10; //
        pub const m_PlayerImpactSound: usize = 0x1C20; //
        pub const m_ImpactModifier: usize = 0x1C30; //
        pub const m_DamagePreventionModifier: usize = 0x1C40; //
        pub const m_RollingModifier: usize = 0x1C50; //
        pub const m_flTransformToBallTime: usize = 0x1C60; //
        pub const m_flTransformFromBallTime: usize = 0x1C64; //
        pub const m_flAirTurnRatio: usize = 0x1C68; //
        pub const m_flWallTurnRatioMax: usize = 0x1C6C; //
        pub const m_flWallTurnRatioMin: usize = 0x1C70; //
        pub const m_flTurnRatio: usize = 0x1C74; //
        pub const m_flDefaultBallSpeed: usize = 0x1C78; //
        pub const m_flFastBallSpeed: usize = 0x1C7C; //
        pub const m_flSpeedAccel: usize = 0x1C80; //
        pub const m_flSpeedDeccel: usize = 0x1C84; //
        pub const m_flElasticity: usize = 0x1C88; //
        pub const m_flWallCheckGroundOffset: usize = 0x1C8C; //
        pub const m_flWallPauseTime: usize = 0x1C90; //
        pub const m_flWallAngleMin: usize = 0x1C94; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Nearby_Enemy_Boost {
    }
    // Parent: CCitadelModifierVData
    // Field count: 6
    pub mod CCitadel_Modifier_Base_BuildupVData {
        pub const m_bUseBaseWeaponCycleTimeForDelay: usize = 0x638; //
        pub const m_flCycleTimeDelayAdd: usize = 0x63C; //
        pub const m_flBuildUpDecayDelay: usize = 0x640; //
        pub const m_eBuildupMode: usize = 0x644; //
        pub const m_bBuildupAffectedByEffectiveness: usize = 0x648; //
        pub const m_bPassBuildupEffectivenessToFillModifier: usize = 0x649; //
    }
    // Parent: C_CitadelItemPickup
    // Field count: 2
    pub mod CCitadelItemPickupRejuv {
        pub const m_CCitadelAbilityComponent: usize = 0xB30; //
        pub const m_bPickedUp: usize = 0xCD0; //
    }
    // Parent: C_BaseToggle
    // Field count: 0
    pub mod C_FuncMoveLinear {
    }
    // Parent: C_BaseModelEntity
    // Field count: 24
    pub mod C_Beam {
        pub const m_flFrameRate: usize = 0x858; //
        pub const m_flHDRColorScale: usize = 0x85C; //
        pub const m_flFireTime: usize = 0x860; //
        pub const m_flDamage: usize = 0x864; //
        pub const m_nNumBeamEnts: usize = 0x868; //
        pub const m_queryHandleHalo: usize = 0x86C; //
        pub const m_hBaseMaterial: usize = 0x890; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_nHaloIndex: usize = 0x898; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_nBeamType: usize = 0x8A0; //
        pub const m_nBeamFlags: usize = 0x8A4; //
        pub const m_hAttachEntity: usize = 0x8A8; //
        pub const m_nAttachIndex: usize = 0x8D0; //
        pub const m_fWidth: usize = 0x8DC; //
        pub const m_fEndWidth: usize = 0x8E0; //
        pub const m_fFadeLength: usize = 0x8E4; //
        pub const m_fHaloScale: usize = 0x8E8; //
        pub const m_fAmplitude: usize = 0x8EC; //
        pub const m_fStartFrame: usize = 0x8F0; //
        pub const m_fSpeed: usize = 0x8F4; //
        pub const m_flFrame: usize = 0x8F8; //
        pub const m_nClipStyle: usize = 0x8FC; //
        pub const m_bTurnedOff: usize = 0x900; //
        pub const m_vecEndPos: usize = 0x904; //
        pub const m_hEndEntity: usize = 0x910; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_ZiplineBoostVData {
        pub const m_flRampUpTime: usize = 0x638; //
        pub const m_flPercentageSpeedIncrease: usize = 0x63C; //
        pub const m_cameraSequenceStartBoost: usize = 0x640; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 0
    pub mod C_ItemAmmo {
    }
    // Parent: CCitadelModifier
    // Field count: 3
    pub mod CCitadel_Modifier_Gunslinger_DemonCarbine {
        pub const m_nBulletCount: usize = 0xC0; //
        pub const m_flElapsedPct: usize = 0xC4; //
        pub const m_nFullyChargedParticle: usize = 0xC8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_LashGrappleTarget {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Snipe_Glow {
        pub const m_nFXIndex: usize = 0xC0; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_TechOverflowProcWatcher {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Tokamak_HeatSinks {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 6
    pub mod CCitadel_Ability_Tengu_AirLift {
        pub const m_hGrabTarget: usize = 0xCA0; //
        pub const m_nHoldBombEffect: usize = 0xCA4; //
        pub const m_eFlightState: usize = 0xEA0; //
        pub const m_bIsGrabbing: usize = 0xEA1; //
        pub const m_bIsHoldingBomb: usize = 0xEA2; //
        pub const m_flCurrentSpeed: usize = 0xEA4; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Nikuman {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityChronoSwapVData {
        pub const m_BubbleMoveModifier: usize = 0x1590; //
        pub const m_strSwapStarted: usize = 0x15A0; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Frenzy_MoveSpeed {
        pub const m_flMoveSpeedPerStack: usize = 0xC0; //
    }
    // Parent: C_BaseEntity
    // Field count: 4
    pub mod CCitadelSpectateDirectedCamera {
        pub const m_flCameraDist: usize = 0x564; //
        pub const m_flCameraPitch: usize = 0x568; //
        pub const m_flCameraHeight: usize = 0x56C; //
        pub const m_hTarget: usize = 0x570; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 11
    pub mod CAbilityExplosiveBarrelVData {
        pub const m_BarrelExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_MirvExplodeParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BarrelArmedParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BarrelReadyToExplodeParticle: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExplodeSound: usize = 0x1910; //
        pub const m_strMirvExplodeSound: usize = 0x1920; //
        pub const m_strRiccochetSound: usize = 0x1930; //
        pub const m_strBarrelSoundLp: usize = 0x1940; //
        pub const m_strBarrelLaunchSound: usize = 0x1950; //
        pub const m_strBarrelMeleedSound: usize = 0x1960; //
        pub const m_strBarrelArmedSound: usize = 0x1970; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 7
    pub mod CCitadel_Ability_MobileResupplyVData {
        pub const m_flResupplyForceScale: usize = 0x1590; //
        pub const m_flResupplyUp: usize = 0x1594; //
        pub const m_strKilledSound: usize = 0x1598; //
        pub const m_AuraModifier: usize = 0x15A8; //
        pub const m_DispenserModel: usize = 0x15B8; //
        pub const m_SprayParticle: usize = 0x1698; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DestroyedParticle: usize = 0x1778; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Fervor_VData {
        pub const m_FervorParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BonusesModifier: usize = 0x718; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_WeaponPowerForHealth {
        pub const m_flHealthDrained: usize = 0xC0; //
    }
    // Parent: CCitadel_Modifier_BaseShield
    // Field count: 0
    pub mod CCitadel_Modifier_RegeneratingBulletShield {
    }
    // Parent: C_DynamicProp
    // Field count: 0
    pub mod C_DynamicPropAlias_prop_dynamic_override {
    }
    // Parent: CCitadel_Item_Bubble
    // Field count: 0
    pub mod CCitadel_Item_Stasis_Bomb {
    }
    // Parent: C_PointEntity
    // Field count: 9
    pub mod C_EnvScreenOverlay {
        pub const m_iszOverlayNames: usize = 0x560; //
        pub const m_flOverlayTimes: usize = 0x5B0; //
        pub const m_flStartTime: usize = 0x5D8; //
        pub const m_iDesiredOverlay: usize = 0x5DC; //
        pub const m_bIsActive: usize = 0x5E0; //
        pub const m_bWasActive: usize = 0x5E1; //
        pub const m_iCachedDesiredOverlay: usize = 0x5E4; //
        pub const m_iCurrentOverlay: usize = 0x5E8; //
        pub const m_flCurrentOverlayTime: usize = 0x5EC; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_Mirage_FireScarabs_HealthLoss_VData {
        pub const m_SiphonParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HealModifier: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CAbilityTokamakHeatSinksInherentVData {
        pub const m_HotTracerParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HotWeaponFxParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strHotWeaponShootSound: usize = 0x1750; //
        pub const m_strOverheatRed: usize = 0x1760; //
        pub const m_strOverheatFull: usize = 0x1770; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 12
    pub mod CCitadel_Modifier_TangoTether_TetherVData {
        pub const m_TetherSound: usize = 0x638; //
        pub const m_HealSound: usize = 0x648; //
        pub const m_HitIndicator: usize = 0x658; //
        pub const m_GrappleHitSound: usize = 0x668; //
        pub const m_BuffModifier: usize = 0x678; //
        pub const m_DisconnectingModifier: usize = 0x688; //
        pub const m_DisconnectedModifier: usize = 0x698; //
        pub const m_LockedTargetModifier: usize = 0x6A8; //
        pub const m_flMinConnectTime: usize = 0x6B8; //
        pub const m_flDisconnectDistanceBuffer: usize = 0x6BC; //
        pub const m_flCandidateCloserDistance: usize = 0x6C0; //
        pub const m_flTargetAwayDistance: usize = 0x6C4; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 9
    pub mod CCitadel_Ability_WreckerTeleport {
        pub const m_hProjectile: usize = 0xCA8; //
        pub const m_flArrowSpeed: usize = 0xCAC; //
        pub const m_flSnapAnglesBackTime: usize = 0xCB0; //
        pub const m_flCastTimeDamage: usize = 0xCB4; //
        pub const m_flCastTime: usize = 0xCB8; //
        pub const m_bNeedsExplosion: usize = 0xCBC; //
        pub const m_vProjectileRemovedOrigin: usize = 0xCC0; //
        pub const m_angCasterAnglesAtCastTime: usize = 0xCCC; //
        pub const m_flTravelDistance: usize = 0xCD8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_SnipeGlowVData {
        pub const m_GlowParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 5
    pub mod CCitadel_Modifier_ChargeDragEnemy {
        pub const m_qRelativeOffset: usize = 0xC0; //
        pub const m_flRelativeDist: usize = 0xCC; //
        pub const m_flMaxDist: usize = 0xD0; //
        pub const m_vecOffsetDir: usize = 0xD4; //
        pub const m_vecStartPosition: usize = 0xE0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_LightningStrikeAreaVData {
        pub const m_GroundParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StrikeParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_GroundParticleFriendly: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StrikeParticleFriendly: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flHeight: usize = 0x9B8; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_Containment_Victim {
        pub const m_flTetherRadius: usize = 0xC0; //
        pub const m_vecOrigin: usize = 0xC4; //
    }
    // Parent: CBaseAnimGraph
    // Field count: 5
    pub mod CCitadel_GrandFinaleStage {
        pub const m_vStartPos: usize = 0xAF8; //
        pub const m_vEndPos: usize = 0xB04; //
        pub const m_flStartEmitTime: usize = 0xB10; //
        pub const m_flEndEmitTime: usize = 0xB14; //
        pub const m_nTouchCount: usize = 0xB18; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_VacuumAura {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadelModifierAura_Default {
    }
    // Parent: C_ModelPointEntity
    // Field count: 17
    pub mod C_PointWorldText {
        pub const m_bForceRecreateNextUpdate: usize = 0x860; //
        pub const m_messageText: usize = 0x878; //
        pub const m_FontName: usize = 0xA78; //
        pub const m_BackgroundMaterialName: usize = 0xAB8; //
        pub const m_bEnabled: usize = 0xAF8; //
        pub const m_bFullbright: usize = 0xAF9; //
        pub const m_flWorldUnitsPerPx: usize = 0xAFC; //
        pub const m_flFontSize: usize = 0xB00; //
        pub const m_flDepthOffset: usize = 0xB04; //
        pub const m_bDrawBackground: usize = 0xB08; //
        pub const m_flBackgroundBorderWidth: usize = 0xB0C; //
        pub const m_flBackgroundBorderHeight: usize = 0xB10; //
        pub const m_flBackgroundWorldToUV: usize = 0xB14; //
        pub const m_Color: usize = 0xB18; //
        pub const m_nJustifyHorizontal: usize = 0xB1C; // PointWorldTextJustifyHorizontal_t
        pub const m_nJustifyVertical: usize = 0xB20; //
        pub const m_nReorientMode: usize = 0xB24; //
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod C_CitadelPortraitWorldCallbackHandler {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_Wraith_RapidFire {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_ShieldImpactVData {
        pub const m_ShieldBreakParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShieldBreakSound: usize = 0x718; //
    }
    // Parent: CEntityComponent
    // Field count: 2
    pub mod CCitadelHeroComponent {
        pub const m_nHeroID: usize = 0x14; //
        pub const m_nHeroLoading: usize = 0x18; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_Charge_Mastery {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 10
    pub mod CCitadel_Ability_Nano_Pounce_Instant {
        pub const m_bActive: usize = 0xED0; //
        pub const m_hCurrentTarget: usize = 0xED4; //
        pub const m_hLastCastTarget: usize = 0xED8; //
        pub const m_vStartPosition: usize = 0xEDC; //
        pub const m_vDeparturePosition: usize = 0xEE8; //
        pub const m_flDepartureTime: usize = 0xEF8; //
        pub const m_flArrivalTime: usize = 0xF10; //
        pub const m_vLastKnownSafePos: usize = 0xF28; //
        pub const m_bIsFirstCastCompleted: usize = 0xF36; //
        pub const m_tDoubleCastWindow: usize = 0xF38; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Infuser_VData {
        pub const m_BuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: None
    // Field count: 1
    pub mod CCitadelAutoScaledTime {
        pub const m_flTime: usize = 0x8; //
    }
    // Parent: CAI_CitadelNPCVData
    // Field count: 8
    pub mod CNPC_MidBossVData {
        pub const m_iStartingHealth: usize = 0x10E8; //
        pub const m_iHealthGainPerMinute: usize = 0x10EC; //
        pub const m_flAggroTime: usize = 0x10F0; //
        pub const m_DyingSmallExplosion: usize = 0x10F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DyingFinalExplosion: usize = 0x11D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_flDyingDuration: usize = 0x12B8; //
        pub const m_KnockbackAura: usize = 0x12C0; //
        pub const m_AggroEnemy: usize = 0x12D0; //
    }
    // Parent: CCitadelModifierAuraVData
    // Field count: 1
    pub mod CCitadel_Modifier_Thumper_PullAOE_VData {
        pub const m_AuraParticle: usize = 0x680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Nikuman {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_TechUpgrade_Infuser {
    }
    // Parent: CitadelAbilityVData
    // Field count: 11
    pub mod CCitadel_Ability_Tengu_StoneFormVData {
        pub const m_CastParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StoneFormParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastTargetSound: usize = 0x1830; //
        pub const m_strImpactSound: usize = 0x1840; //
        pub const m_strFallCollideImpactSound: usize = 0x1850; //
        pub const m_DragModifier: usize = 0x1860; //
        pub const m_strTrueFormModel: usize = 0x1870; //
        pub const m_flLandHoldTime: usize = 0x1950; //
        pub const m_flRisingTime: usize = 0x1954; //
        pub const m_flCollideRadius: usize = 0x1958; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CCitadel_Ability_UltComboVData {
        pub const m_MeleeSwingParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_MeleeImpactParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SelfModifier: usize = 0x1750; //
        pub const m_TargetModifier: usize = 0x1760; //
        pub const m_KillCheckModifier: usize = 0x1770; //
        pub const m_flKillCheckWindow: usize = 0x1780; //
    }
    // Parent: CCitadel_Ability_PrimaryWeapon
    // Field count: 0
    pub mod CCitadel_Ability_Shotgun_Astro {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Astro_Rifle_Self {
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_DragEnemyVData {
        pub const m_flForwardOffset: usize = 0x638; //
        pub const m_flVerticalOffset: usize = 0x63C; //
        pub const m_flDragDistance: usize = 0x640; //
        pub const m_flForceDistScale: usize = 0x644; //
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_SilenceProcWatcher {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Item {
        pub const m_vecComponentsConsumed: usize = 0xCA0; // C_NetworkUtlVectorBase<CUtlStringToken>
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Magician_MagicBolt {
        pub const m_vecDeployedProjectiles: usize = 0xCA8; // CUtlVector<CHandle<C_CitadelProjectile>>
        pub const m_iCurrentRedirects: usize = 0xCC0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CAbilityDustStormVData {
        pub const m_DustStormAura: usize = 0x1590; //
        pub const m_GrenadeTrailModifier: usize = 0x15A0; //
    }
    // Parent: CitadelItemVData
    // Field count: 1
    pub mod CCitadel_UtilityUpgrade_DebuffImmunityVData {
        pub const m_DebuffImmunityModifier: usize = 0x15D8; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_UtilityUpgrade_RocketBootsVData {
        pub const m_LaunchParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_InAirWatcherModifier: usize = 0x16B8; //
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProcVData
    // Field count: 2
    pub mod CCitadel_Modifier_CritShotVData {
        pub const m_SlowModifier: usize = 0x768; //
        pub const m_CritSound: usize = 0x778; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifier_WarpStone_Caster_VData {
        pub const m_playerBuffSelf: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_SiphonBullets_HealthLoss_VData {
        pub const m_SiphonParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HealModifier: usize = 0x718; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_BerserkerVData {
        pub const m_BerserkerSound: usize = 0x638; //
        pub const m_StackModifier: usize = 0x648; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_Delayed_Stun {
        pub const m_hRingEffect: usize = 0xC0; //
        pub const m_flRadius: usize = 0xC4; //
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_HeroUpgradeBonuses {
        pub const m_pOwningPlayer: usize = 0xC0; //
        pub const m_flWeaponPower: usize = 0xC8; //
        pub const m_flArmorPower: usize = 0xCC; //
        pub const m_flTechPower: usize = 0xD0; //
    }
    // Parent: C_DynamicProp
    // Field count: 8
    pub mod C_BasePropDoor {
        pub const m_eDoorState: usize = 0xDA8; //
        pub const m_modelChanged: usize = 0xDAC; //
        pub const m_bLocked: usize = 0xDAD; //
        pub const m_bNoNPCs: usize = 0xDAE; //
        pub const m_closedPosition: usize = 0xDB0; //
        pub const m_closedAngles: usize = 0xDBC; //
        pub const m_hMaster: usize = 0xDC8; //
        pub const m_vWhereToSetLightingOrigin: usize = 0xDCC; //
    }
    // Parent: C_PointEntity
    // Field count: 0
    pub mod C_PointEntityAlias_info_target_portrait_root {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_GarbageAura {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_PhantomStrike {
    }
    // Parent: CCitadel_Ability_PrimaryWeapon
    // Field count: 0
    pub mod CCitadel_Ability_Shotgun_Astro_Backwards {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_SelfVacuum {
    }
    // Parent: CCitadelModifier
    // Field count: 7
    pub mod CCitadel_Modifier_ShieldTracker_Base {
        pub const m_vecShield: usize = 0xC0; //
        pub const m_flShieldDamageTime: usize = 0xCC; //
        pub const m_flShieldBreakTime: usize = 0xD0; //
        pub const m_flShieldBreakHealAmount: usize = 0xD4; //
        pub const m_flShieldBreakHealDone: usize = 0xD8; //
        pub const m_bShieldHealingAfterBreak: usize = 0xDC; //
        pub const m_bForceRegen: usize = 0xDD; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod C_NPC_MidBoss {
    }
    // Parent: CBaseAnimGraph
    // Field count: 0
    pub mod CCitadelItemMetal {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Projectile_Rolling_FireBall {
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 4
    pub mod CCitadel_Modifier_VacuumAuraTarget {
        pub const m_flMaxDist: usize = 0x138; //
        pub const m_vecOffsetDir: usize = 0x13C; //
        pub const m_vecStartPosition: usize = 0x148; //
        pub const m_flAOERadius: usize = 0x154; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CModifier_CloakingDevice_Active_Ambush {
        pub const m_nAmbushParticle: usize = 0xC0; //
    }
    // Parent: CitadelItemVData
    // Field count: 7
    pub mod CCitadel_ArmorUpgrade_AblativeCoatVData {
        pub const m_RestoreEffectModifier: usize = 0x15D8; //
        pub const m_OnTakeDamageEffectModifier: usize = 0x15E8; //
        pub const m_OnBreakEffectModifier: usize = 0x15F8; //
        pub const m_ResistBuffModifier: usize = 0x1608; //
        pub const m_flOnTakeDamageEffectDuration: usize = 0x1618; //
        pub const m_flOnBreakEffectDuration: usize = 0x161C; //
        pub const m_flOnRestoreEffectDuration: usize = 0x1620; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ActiveDisarm_SpiritSteal {
    }
    // Parent: CCitadel_Modifier_BaseEventProc
    // Field count: 0
    pub mod CCitadel_Modifier_SpiritSnatch {
    }
    // Parent: C_BreakableProp
    // Field count: 6
    pub mod C_PhysPropClientside {
        pub const m_flTouchDelta: usize = 0xC80; //
        pub const m_fDeathTime: usize = 0xC84; //
        pub const m_inertiaScale: usize = 0xC88; //
        pub const m_vecDamagePosition: usize = 0xC8C; //
        pub const m_vecDamageDirection: usize = 0xC98; //
        pub const m_nDamageType: usize = 0xCA4; //
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod C_Projectile_Perched_Predator {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_FireBeetles_Buff {
    }
    // Parent: CCitadel_Ability_PrimaryWeapon
    // Field count: 0
    pub mod CCitadel_Ability_Nano_PrimaryWeapon {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_Viscous_Telepunch {
        pub const m_vecTeleportPosition: usize = 0xF08; //
        pub const m_vecTeleportPositionNormal: usize = 0xF14; //
        pub const m_eTelepunchState: usize = 0xF20; //
        pub const m_flNextStateTime: usize = 0xF24; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_GooGrenade {
        pub const m_vecPuddleModifiers: usize = 0xCA0; // CUtlVector<CHandle<C_BaseEntity>>
        pub const m_LastDetonateTime: usize = 0xEB0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CCitadel_Ability_ViscousWeapon_Alt_VData {
        pub const m_strChargingParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ImpactParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FiringParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionSound: usize = 0x1830; //
        pub const m_ChargeSound: usize = 0x1840; //
        pub const m_ShootSound: usize = 0x1850; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_Chrono_TimeWall {
        pub const m_hChargingParticle: usize = 0xCA0; //
        pub const m_vSpawnPos: usize = 0xCA4; //
        pub const m_qAngles: usize = 0xCB0; //
        pub const m_bAirCast: usize = 0xCBC; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ServerOnly {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadelBaseDashCastAbility {
        pub const m_hAbilityToTrigger: usize = 0xCA0; //
        pub const m_flDashCastStartTime: usize = 0xCA4; //
        pub const m_vDashCastDir: usize = 0xCA8; //
    }
    // Parent: CBaseFilter
    // Field count: 1
    pub mod CFilterProximity {
        pub const m_flRadius: usize = 0x5B8; //
    }
    // Parent: C_BaseEntity
    // Field count: 4
    pub mod C_SoundAreaEntityBase {
        pub const m_bDisabled: usize = 0x560; //
        pub const m_bWasEnabled: usize = 0x568; //
        pub const m_iszSoundAreaType: usize = 0x570; //
        pub const m_vPos: usize = 0x578; //
    }
    // Parent: C_BaseEntity
    // Field count: 3
    pub mod C_FogController {
        pub const m_fog: usize = 0x560; //
        pub const m_bUseAngles: usize = 0x5C8; //
        pub const m_iChangedVariables: usize = 0x5CC; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_FireScarabs_HealthLoss {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_ShivDashVData {
        pub const m_DashParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DashEchoParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DashTrailParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 10
    pub mod CCitadel_Ability_BurrowVData {
        pub const m_ExplodeParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BurrowStartParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BurrowEndParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BurrowInGroundParticle: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BurrowModifier: usize = 0x1910; //
        pub const m_SpinModifier: usize = 0x1920; //
        pub const m_strBurrowEndSound: usize = 0x1930; //
        pub const m_flChannelEndEnemyPopUpForce: usize = 0x1940; //
        pub const m_flChannelEndEnemyPopUpCylinderHeight: usize = 0x1944; //
        pub const m_cameraSpinStart: usize = 0x1948; //
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProc
    // Field count: 0
    pub mod CCitadel_Modifier_CritShot {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_Savior_VData {
        pub const m_BuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TrailParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Cadence_AnthemAOE {
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 3
    pub mod CCitadel_Modifier_GarbageAuraTargetModifierVData {
        pub const m_flOuterSpeedScale: usize = 0x718; //
        pub const m_flSpeedScaleBias: usize = 0x71C; //
        pub const m_TargetLoopingSound: usize = 0x720; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 16
    pub mod CAbilityWreckerTeleportVData {
        pub const m_SpectatingProjectileParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplosionParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChannelParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastParticle: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ArrowOffsetX: usize = 0x1910; //
        pub const m_ArrowCameraDistance: usize = 0x1914; //
        pub const m_ArrowCameraHeightOffset: usize = 0x1918; //
        pub const m_ArrowInitialPitch: usize = 0x191C; //
        pub const m_GuidingModifier: usize = 0x1920; //
        pub const m_DebuffModifier: usize = 0x1930; //
        pub const m_strExplodeSound: usize = 0x1940; //
        pub const m_flTrackAmount: usize = 0x1950; //
        pub const m_flSpeedAccel: usize = 0x1954; //
        pub const m_flSpeedDeccel: usize = 0x1958; //
        pub const m_flBaseProjectileSpeed: usize = 0x195C; //
        pub const m_flMaxProjectileSpeed: usize = 0x1960; //
    }
    // Parent: CitadelItemVData
    // Field count: 2
    pub mod CCitadel_WeaponUpgrade_SpellslingerHeadshots_VData {
        pub const m_HeadshotDebuffModifier: usize = 0x15D8; //
        pub const m_ImpactParticle: usize = 0x15E8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Savior {
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Damage_Taken_Reduction_Handicap {
        pub const m_flValue: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Objective_Regen {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_BulletResistReductionStackVData {
        pub const m_bSelfish: usize = 0x638; //
    }
    // Parent: CPlayerPawnComponent
    // Field count: 0
    pub mod CPlayer_ItemServices {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_ActiveBulletShield {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadelBaseLockonAbility {
        pub const m_vecLockonTargets: usize = 0xDB8; // C_UtlVectorEmbeddedNetworkVar<LockonTarget_t>
        pub const m_LockOnStartTime: usize = 0xE20; //
        pub const m_nTargetingLightEffect: usize = 0xE28; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CAbility_Rutger_CheatDeath {
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CCitadel_Modifier_BoucePadVData {
        pub const m_StompParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strImpactSound: usize = 0x718; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 5
    pub mod CAbilityStompVData {
        pub const m_StompParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strStompExplosionSound: usize = 0x1670; //
        pub const m_strCastDelayLocalPlayerSound: usize = 0x1680; //
        pub const m_DebuffModifier: usize = 0x1690; //
        pub const m_BulletResistModifier: usize = 0x16A0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_PassiveBeefy {
    }
    // Parent: CitadelAbilityVData
    // Field count: 13
    pub mod CAbilityStormCloudVData {
        pub const m_AoEPreviewParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_StormCloudModifier: usize = 0x1670; //
        pub const m_LightningStrikeAOEModifier: usize = 0x1680; //
        pub const m_strLightningStrikeCast: usize = 0x1690; //
        pub const m_flOscillateFrequency: usize = 0x16A0; //
        pub const m_flOscillateSpeed: usize = 0x16A4; //
        pub const m_flOscillateSpeedStart: usize = 0x16A8; //
        pub const m_flOscillateStartOffset: usize = 0x16AC; //
        pub const m_flAirDrag: usize = 0x16B0; //
        pub const m_flFlightAirDrag: usize = 0x16B4; //
        pub const m_flFlightLateralMoveSpeed: usize = 0x16B8; //
        pub const m_flVerticalMoveSpeedPercent: usize = 0x16BC; //
        pub const m_flAirAcceleration: usize = 0x16C0; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Tech_Bleed {
        pub const m_hRingEffect: usize = 0xC0; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_HoldingGoldenIdol {
        pub const m_iIdolParticle: usize = 0x130; //
        pub const m_nGoldValue: usize = 0x134; //
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_FuncBrush {
    }
    // Parent: C_BaseEntity
    // Field count: 8
    pub mod CInfoWorldLayer {
        pub const m_pOutputOnEntitiesSpawned: usize = 0x560; //
        pub const m_worldName: usize = 0x588; //
        pub const m_layerName: usize = 0x590; //
        pub const m_bWorldLayerVisible: usize = 0x598; //
        pub const m_bEntitiesSpawned: usize = 0x599; //
        pub const m_bCreateAsChildSpawnGroup: usize = 0x59A; //
        pub const m_hLayerSpawnGroup: usize = 0x59C; //
        pub const m_bWorldLayerActuallyVisible: usize = 0x5A0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CModifier_Operative_Revelation_Caster_VData {
        pub const m_AuraModifier: usize = 0x638; //
        pub const m_ShieldParticle: usize = 0x648; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 7
    pub mod CCitadel_Ability_ShivDaggerVData {
        pub const m_DamageDebuffModifier: usize = 0x1590; //
        pub const m_SlowDebuffModifier: usize = 0x15A0; //
        pub const m_DaggerStuckParticle: usize = 0x15B0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerImpactParticle: usize = 0x1690; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DaggerExplodeParticle: usize = 0x1770; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDaggerHitSound: usize = 0x1850; //
        pub const m_strDaggerExplodeSound: usize = 0x1860; //
    }
    // Parent: CCitadel_Modifier_Stunned
    // Field count: 2
    pub mod CCitadel_Modifier_PsychicLift {
        pub const m_vecFloatDest: usize = 0x138; //
        pub const m_bImpacted: usize = 0x144; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_LightningStrikeArea {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_NearDeathFX {
    }
    // Parent: CCitadelModifierVData
    // Field count: 3
    pub mod CCitadel_Modifier_ZiplineSpeedVData {
        pub const m_flPercentageMultiplierStart: usize = 0x638; //
        pub const m_flPercentageMultiplierEnd: usize = 0x63C; //
        pub const m_flRampUpTime: usize = 0x640; //
    }
    // Parent: C_BaseCombatCharacter
    // Field count: 0
    pub mod C_NetTestBaseCombatCharacter {
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod C_Citadel_PestilenceDroneDispenser {
    }
    // Parent: CBaseAnimGraph
    // Field count: 4
    pub mod C_Citadel_BreakblePropPickup {
        pub const m_bActive: usize = 0xAF8; //
        pub const m_sPickupName: usize = 0xB00; //
        pub const m_nNameOffset: usize = 0xB08; //
        pub const m_sAmbientNoise: usize = 0xB10; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 2
    pub mod CGameModifier_FireUserEntityIOVData {
        pub const m_FireOnAdded: usize = 0x638; //
        pub const m_FireOnRemoved: usize = 0x63C; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_GangActivity_Cancel {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_Shakedown_TargetVData {
        pub const m_RootModifier: usize = 0x1590; //
        pub const m_PulseModifier: usize = 0x15A0; //
    }
    // Parent: CCitadel_Ability_PrimaryWeaponVData
    // Field count: 4
    pub mod CCitadel_Ability_Nano_PrimaryWeaponVData {
        pub const m_EscapeModifier: usize = 0x15E0; //
        pub const m_SlashEffectParticle: usize = 0x15F0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strExpireSound: usize = 0x16D0; //
        pub const m_cameraSequenceInShadow: usize = 0x16E0; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_WingBlastPush {
        pub const m_vPush: usize = 0xC0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 7
    pub mod CCitadel_Ability_Mantle {
        pub const m_flVertOffset: usize = 0xCA0; //
        pub const m_flHorizGap: usize = 0xCA4; //
        pub const m_vStartPos: usize = 0xCA8; //
        pub const m_vTargetPos: usize = 0xCB4; //
        pub const m_angFacing: usize = 0xCC0; //
        pub const m_nMantleTypeIndex: usize = 0xCCC; //
        pub const m_flStartTime: usize = 0xCD0; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 4
    pub mod CCitadel_Modifier_NearDeathFXVData {
        pub const m_EnemyNearDeathParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FriendlyNearDeathParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_sSelfDestructStart: usize = 0x7F8; //
        pub const m_sSelfDestructEnd: usize = 0x808; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod C_NPC_CarpetBombDrone {
    }
    // Parent: C_CitadelItemPickup
    // Field count: 0
    pub mod CCitadelItemPickupIdol {
    }
    // Parent: C_Citadel_BreakblePropPickup
    // Field count: 0
    pub mod C_Citadel_BreakblePropModifierPickup {
    }
    // Parent: CBaseAnimGraph
    // Field count: 2
    pub mod C_PhysMagnet {
        pub const m_aAttachedObjectsFromServer: usize = 0xAF8; //
        pub const m_aAttachedObjects: usize = 0xB10; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: CitadelAbilityVData
    // Field count: 6
    pub mod CCitadel_Ability_Magician_AnimalCurseVData {
        pub const m_CurseModifier: usize = 0x1590; //
        pub const m_AirDampingModifier: usize = 0x15A0; //
        pub const m_TargetWarningSound: usize = 0x15B0; //
        pub const m_ProjectileImpactParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TargetWarningParticle: usize = 0x16A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ProjectileExplodeParticle: usize = 0x1780; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_AirLiftExplodingAlly {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 2
    pub mod CCitadel_Ability_Wrecker_Ultimate {
        pub const m_angBeamAngles: usize = 0xCC0; //
        pub const m_bNeedsBeamReset: usize = 0xCD8; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 8
    pub mod CCitadelModifierChronoPulseGrenadePulseAreaVData {
        pub const m_DebuffModifier: usize = 0x638; //
        pub const m_SlowModifier: usize = 0x648; //
        pub const m_PreviewRingParticle: usize = 0x658; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AreaEffect: usize = 0x738; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strArmingSound: usize = 0x818; //
        pub const m_strArmedSound: usize = 0x828; //
        pub const m_strLoopingSound: usize = 0x838; //
        pub const m_strHitSound: usize = 0x848; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 3
    pub mod CCitadel_Ability_HornetLeap {
        pub const m_bLeaping: usize = 0xCA2; //
        pub const m_flLeapStartTime: usize = 0xCA4; //
        pub const m_nFXIndex: usize = 0xCA8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ChainLightningEffect {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_VexBarrier_Shield {
    }
    // Parent: None
    // Field count: 3
    pub mod CEntityInstance {
        pub const m_iszPrivateVScripts: usize = 0x8; //
        pub const m_pEntity: usize = 0x10; //
        pub const m_CScriptComponent: usize = 0x30; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_Frenzy {
    }
    // Parent: C_PointClientUIWorldPanel
    // Field count: 1
    pub mod C_InWorldKeyBindPanel {
        pub const m_hPlayer: usize = 0xAC0; //
    }
    // Parent: CBasePlayerController
    // Field count: 31
    pub mod CCitadelPlayerController {
        pub const m_ePlayState: usize = 0x768; //
        pub const m_iGuidedBotMatchLastHits: usize = 0x76C; //
        pub const m_iGuidedBotMatchOrbsSecured: usize = 0x770; //
        pub const m_iGuidedBotMatchOrbsDenied: usize = 0x774; //
        pub const m_iGuidedBotMatchDamageToGuardians: usize = 0x778; //
        pub const m_iGuidedBotMatchDamageToPlayers: usize = 0x77C; //
        pub const m_iGuidedBotMatchDamageTaken: usize = 0x780; //
        pub const m_iGuidedBotMatchNetWorth: usize = 0x784; //
        pub const m_iGuidedBotMatchModsPurchased: usize = 0x788; //
        pub const m_iGuidedBotMatchAbilityUpgrades: usize = 0x78C; //
        pub const m_flGuideBotMatchLastTaskNagVO: usize = 0x790; //
        pub const m_flGuideBotLastTimeTaskCompleted: usize = 0x794; //
        pub const m_eGuidedBotMatchObjective: usize = 0x798; //
        pub const m_nCurrentRank: usize = 0x79C; //
        pub const m_nAssignedLane: usize = 0x7A0; //
        pub const m_nOriginalLaneAssignment: usize = 0x7A1; //
        pub const m_bIsKingPanda: usize = 0x7A2; //
        pub const m_bBotDisconnectTakeover: usize = 0x7A3; //
        pub const m_bInTeamChat: usize = 0x7A4; //
        pub const m_bInPartyChat: usize = 0x7A5; //
        pub const m_unHeroBuildID: usize = 0x7A8; //
        pub const m_bLaneSwapLocked: usize = 0x7AC; //
        pub const m_vecLaneSwapRequests: usize = 0x7B0; //
        pub const m_vecLaneSwapRejects: usize = 0x7C8; //
        pub const m_vecMutedPlayers: usize = 0x7E0; //
        pub const m_hHeroPawn: usize = 0x7F8; //
        pub const m_PlayerDataGlobal: usize = 0x830; //
        pub const m_nDeathReplayAvailable: usize = 0xAB8; //
        pub const m_unLobbyPlayerSlot: usize = 0xAB9; //
        pub const m_bHasCheckedFriendName: usize = 0xABA; //
        pub const m_sFriendName: usize = 0xAC0; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_GenericPerson_2 {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_Ghost_BloodShards {
        pub const m_vecDamagedTargets: usize = 0xDF0; // CUtlVector<CHandle<C_BaseEntity>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CCitadel_Modifier_Intrinsic_BaseVData {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod CCitadel_Projectile_Cyclone {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadelModifier_Viscous_Goo_Aura {
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_Gunslinger_SpreadingFireVData {
        pub const m_ImpactParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_FireDebuffModifier: usize = 0x1670; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Intimidated {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_TargetPracticeEnemy {
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod C_PortraitWorldCallbackHandler {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierPowerGeneratorVData {
        pub const m_EffectToTitan: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifierVData
    // Field count: 7
    pub mod CCitadel_Modifier_VoidSphereVData {
        pub const m_TeleportStartParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportEndParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportTrailParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportModelParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffModifier: usize = 0x9B8; //
        pub const m_flPreTeleportDuration: usize = 0x9C8; //
        pub const m_strAmbientLoopingLocalPlayerSound: usize = 0x9D0; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 7
    pub mod CCitadel_Ability_Chrono_TimeWallVData {
        pub const m_AuraModifier: usize = 0x1590; //
        pub const m_TimeWallParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TimeWallChargeParticle: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TimeWallHitParticle: usize = 0x1760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TimeWallHitTimerParticle: usize = 0x1840; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strWallCreated: usize = 0x1920; //
        pub const m_strChargeUpSound: usize = 0x1930; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_GhostBloodShardDebuffVData {
        pub const m_BloodShardDebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CCitadel_Modifier_DPSTracker {
        pub const m_flProgress: usize = 0xC0; //
        pub const m_flDistToTarget: usize = 0xC4; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Synth_Blitz {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Urn_Debuff {
    }
    // Parent: CitadelAbilityVData
    // Field count: 3
    pub mod CAbilityChargedShotVData {
        pub const m_ChannelParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ChannelStartParticle: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ShootParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Aerial_Assault {
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Dust_Storm_Aura {
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_Item_CheatDeath {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Fear {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_AirLift_Explode_Target {
    }
    // Parent: CitadelAbilityVData
    // Field count: 11
    pub mod CAbilityHookVData {
        pub const m_SelfModifier: usize = 0x1590; //
        pub const m_TargetModifier: usize = 0x15A0; //
        pub const m_BulletAmpModifier: usize = 0x15B0; //
        pub const m_ShieldModifier: usize = 0x15C0; //
        pub const m_HookOutParticle: usize = 0x15D0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strHookSuccessSound: usize = 0x16B0; //
        pub const m_strHookAllySound: usize = 0x16C0; //
        pub const m_strHookMissSound: usize = 0x16D0; //
        pub const m_strHookImpactGeoSound: usize = 0x16E0; //
        pub const m_SelfBuffCastSound: usize = 0x16F0; //
        pub const m_flTrooperHitRadius: usize = 0x1700; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 4
    pub mod CCitadel_Ability_IcePath {
        pub const m_bIcePathing: usize = 0xD10; //
        pub const m_qLastAngles: usize = 0xD14; //
        pub const m_vLastVelocity: usize = 0xD20; //
        pub const m_bFirstMovementTick: usize = 0xD2C; //
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_LearningHeroAbility {
        pub const m_sDescription: usize = 0xC0; //
    }
    // Parent: None
    // Field count: 33
    pub mod CGameSceneNode {
        pub const m_nodeToWorld: usize = 0x10; //
        pub const m_pOwner: usize = 0x30; //
        pub const m_pParent: usize = 0x38; //
        pub const m_pChild: usize = 0x40; //
        pub const m_pNextSibling: usize = 0x48; //
        pub const m_hParent: usize = 0x78; //
        pub const m_vecOrigin: usize = 0x88; // CNetworkOriginCellCoordQuantizedVector
        pub const m_angRotation: usize = 0xC0; //
        pub const m_flScale: usize = 0xCC; //
        pub const m_vecAbsOrigin: usize = 0xD0; //
        pub const m_angAbsRotation: usize = 0xDC; //
        pub const m_flAbsScale: usize = 0xE8; //
        pub const m_nParentAttachmentOrBone: usize = 0xEC; //
        pub const m_bDebugAbsOriginChanges: usize = 0xEE; //
        pub const m_bDormant: usize = 0xEF; //
        pub const m_bForceParentToBeNetworked: usize = 0xF0; //
        pub const m_bDirtyHierarchy: usize = 0x0; //
        pub const m_bDirtyBoneMergeInfo: usize = 0x0; //
        pub const m_bNetworkedPositionChanged: usize = 0x0; //
        pub const m_bNetworkedAnglesChanged: usize = 0x0; //
        pub const m_bNetworkedScaleChanged: usize = 0x0; //
        pub const m_bWillBeCallingPostDataUpdate: usize = 0x0; //
        pub const m_bBoneMergeFlex: usize = 0x0; //
        pub const m_nLatchAbsOrigin: usize = 0x0; //
        pub const m_bDirtyBoneMergeBoneToRoot: usize = 0x0; //
        pub const m_nHierarchicalDepth: usize = 0xF3; //
        pub const m_nHierarchyType: usize = 0xF4; //
        pub const m_nDoNotSetAnimTimeInInvalidatePhysicsCount: usize = 0xF5; //
        pub const m_name: usize = 0xF8; //
        pub const m_hierarchyAttachName: usize = 0x138; //
        pub const m_flZOffset: usize = 0x13C; //
        pub const m_flClientLocalScale: usize = 0x140; //
        pub const m_vRenderOrigin: usize = 0x144; //
    }
    // Parent: C_Citadel_BreakblePropPickup
    // Field count: 0
    pub mod C_Citadel_BreakblePropHealthPickup {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierVData_SetModelScale {
        pub const m_flScale: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 2
    pub mod CModifier_Mirage_Tornado_Lift {
        pub const m_vecFloatDest: usize = 0x130; //
        pub const m_vecStartingPos: usize = 0x13C; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CAbility_Rutger_ForceField {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 15
    pub mod CCitadel_Ability_Slide {
        pub const m_flGroundDashSlideTime: usize = 0xCF8; //
        pub const m_flSlowGetupStartTime: usize = 0xD10; //
        pub const m_bShouldTriggerSlowGetup: usize = 0xD14; //
        pub const m_bWantsSlide: usize = 0xD15; //
        pub const m_bAirborneWhenDuckPressed: usize = 0xD16; //
        pub const m_bIsSliding: usize = 0xD17; //
        pub const m_flSpeedAdjust: usize = 0xD18; //
        pub const m_flDuckPressedTime: usize = 0xD1C; //
        pub const m_flSlideChangeTime: usize = 0xD20; //
        pub const m_flSlidingOnFlatStartTime: usize = 0xD24; //
        pub const m_nJumpsThisSlideSession: usize = 0xD28; //
        pub const m_flOnGroundStartTime: usize = 0xD2C; //
        pub const m_flDashSlideStartTime: usize = 0xD30; //
        pub const m_bStartedSlideViaProbeSlope: usize = 0xD34; //
        pub const m_nSlideEffectIndex: usize = 0xD38; //
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 6
    pub mod C_NPC_Boss_Tier3 {
        pub const m_iLane: usize = 0x1420; //
        pub const m_angTargeting1: usize = 0x1428; //
        pub const m_angTargeting2: usize = 0x1440; //
        pub const m_nElectricBeamCasts: usize = 0x1458; //
        pub const m_eAliveState: usize = 0x145C; //
        pub const m_ePhase: usize = 0x1460; //
    }
    // Parent: CCitadelModifier
    // Field count: 8
    pub mod CCitadel_Modifier_MageWalk {
        pub const m_bIsFakeout: usize = 0xC0; //
        pub const m_bTeleported: usize = 0xC1; //
        pub const m_particleStart: usize = 0xC4; //
        pub const m_particleEnd: usize = 0xC8; //
        pub const m_particleTrail: usize = 0xCC; //
        pub const m_vecEndLocation: usize = 0xD0; //
        pub const m_vecStartPosition: usize = 0xDC; //
        pub const m_vecEndLocationCaster: usize = 0xE8; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 1
    pub mod CCitadel_Ability_Astro_Shotgun_Toggle_VData {
        pub const m_BuffModifier: usize = 0x1590; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_SpellShield_Buff {
    }
    // Parent: CCitadel_Modifier_Intrinsic_BaseVData
    // Field count: 1
    pub mod CCitadel_Modifier_MagicStormWatcherVData {
        pub const m_BuffModifier: usize = 0x638; //
    }
    // Parent: CCitadelModifier
    // Field count: 4
    pub mod CCitadel_Modifier_TrooperDisabledInvulnerability {
        pub const m_flBulletResistancePctMax: usize = 0xC0; //
        pub const m_bShieldUp: usize = 0xC4; //
        pub const m_flShieldUpTime: usize = 0xC8; //
        pub const m_trackInfo: usize = 0xCC; //
    }
    // Parent: C_BaseEntity
    // Field count: 3
    pub mod C_EntityFlame {
        pub const m_hEntAttached: usize = 0x560; //
        pub const m_hOldAttached: usize = 0x588; //
        pub const m_bCheapEffect: usize = 0x58C; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_FlameDashGroundAura {
    }
    // Parent: C_BaseModelEntity
    // Field count: 0
    pub mod C_Breakable {
    }
    // Parent: CCitadel_Ability_Melee_Base
    // Field count: 1
    pub mod CCitadel_Ability_Uppercut {
        pub const m_bShouldUseResources: usize = 0xF50; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_PsychicPulse {
        pub const m_vecPulseTargets: usize = 0xCD8; //
    }
    // Parent: C_BaseEntity
    // Field count: 0
    pub mod C_TintController {
    }
    // Parent: CPlayerPawnComponent
    // Field count: 0
    pub mod CPlayer_AutoaimServices {
    }
    // Parent: C_CitadelProjectile
    // Field count: 0
    pub mod CCitadel_Projectile_SpiderProjectile {
    }
    // Parent: CCitadel_Modifier_StunnedVData
    // Field count: 4
    pub mod CCitadel_Modifier_PillarVData {
        pub const m_DebuffParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffStartParticle: usize = 0x7F8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_BuffEndParticle: usize = 0x8D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_PostSleepModifier: usize = 0x9B8; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_WingBlastApply {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_ImmobilizeTrap_Debuff {
    }
    // Parent: CCitadelModifierAura_Cone
    // Field count: 3
    pub mod CCitadel_Modifier_Fathom_ScaldingSpray_Aura {
        pub const m_playerAngles: usize = 0xF0; //
        pub const m_flLastStackTime: usize = 0xFC; //
        pub const m_ConeParticle: usize = 0x100; //
    }
    // Parent: CCitadel_WeaponUpgrade_HeadshotBooster
    // Field count: 0
    pub mod CCitadel_WeaponUpgrade_Headhunter {
    }
    // Parent: C_BaseModelEntity
    // Field count: 24
    pub mod C_Sprite {
        pub const m_hSpriteMaterial: usize = 0x858; // CStrongHandle<InfoForResourceTypeIMaterial2>
        pub const m_hAttachedToEntity: usize = 0x860; //
        pub const m_nAttachment: usize = 0x864; //
        pub const m_flSpriteFramerate: usize = 0x868; //
        pub const m_flFrame: usize = 0x86C; //
        pub const m_flDieTime: usize = 0x870; //
        pub const m_nBrightness: usize = 0x880; //
        pub const m_flBrightnessDuration: usize = 0x884; //
        pub const m_flSpriteScale: usize = 0x888; //
        pub const m_flScaleDuration: usize = 0x88C; //
        pub const m_bWorldSpaceScale: usize = 0x890; //
        pub const m_flGlowProxySize: usize = 0x894; //
        pub const m_flHDRColorScale: usize = 0x898; //
        pub const m_flLastTime: usize = 0x89C; //
        pub const m_flMaxFrame: usize = 0x8A0; //
        pub const m_flStartScale: usize = 0x8A4; //
        pub const m_flDestScale: usize = 0x8A8; //
        pub const m_flScaleTimeStart: usize = 0x8AC; //
        pub const m_nStartBrightness: usize = 0x8B0; //
        pub const m_nDestBrightness: usize = 0x8B4; //
        pub const m_flBrightnessTimeStart: usize = 0x8B8; //
        pub const m_hOldSpriteMaterial: usize = 0x8C0; // CWeakHandle<InfoForResourceTypeIMaterial2>
        pub const m_nSpriteWidth: usize = 0x960; //
        pub const m_nSpriteHeight: usize = 0x964; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CAbilityBullChargeVData {
        pub const m_cameraSequenceImpact: usize = 0x1590; //
        pub const m_ModifierTossAirControlLockout: usize = 0x1618; //
        pub const m_ModifierWeaponPowerIncrease: usize = 0x1628; //
        pub const m_ModifierChargeDragEnemy: usize = 0x1638; //
        pub const m_ModifierBullCharging: usize = 0x1648; //
        pub const m_WallImpactParticle: usize = 0x1658; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strWallSlamSound: usize = 0x1738; //
        pub const m_flWallStunLookAheadDist: usize = 0x1748; //
        pub const m_flEndChargeVelocityScale: usize = 0x174C; //
    }
    // Parent: C_TeamplayRules
    // Field count: 48
    pub mod C_CitadelGameRules {
        pub const m_bFreezePeriod: usize = 0x58; //
        pub const m_fLevelStartTime: usize = 0x5C; //
        pub const m_flGameStartTime: usize = 0x60; //
        pub const m_flRoundStartTime: usize = 0x64; //
        pub const m_eGameState: usize = 0x68; //
        pub const m_hTowerAmber: usize = 0x6C; //
        pub const m_hTowerSapphire: usize = 0x70; //
        pub const m_bEnemyInAmberBase: usize = 0x74; //
        pub const m_bEnemyInSapphireBase: usize = 0x75; //
        pub const m_vMinimapMins: usize = 0x78; //
        pub const m_vMinimapMaxs: usize = 0x84; //
        pub const m_bMatchSafeToAbandon: usize = 0x90; //
        pub const m_bNoDeathEnabled: usize = 0x91; //
        pub const m_bFastCooldownsEnabled: usize = 0x92; //
        pub const m_bStaminaCooldownsEnabled: usize = 0x93; //
        pub const m_bUnlimitedAmmoEnabled: usize = 0x94; //
        pub const m_bInfiniteResourcesEnabled: usize = 0x95; //
        pub const m_bFlexSlotsForcedUnlocked: usize = 0x96; //
        pub const m_eMatchMode: usize = 0x98; //
        pub const m_eGameMode: usize = 0x9C; //
        pub const m_unSpectatorCount: usize = 0xA0; //
        pub const m_hTrooperMinimap: usize = 0xA4; //
        pub const m_hCurrentHeroDrafterRebels: usize = 0xA8; //
        pub const m_hCurrentHeroDrafterCombine: usize = 0xAC; //
        pub const m_bDontUploadStats: usize = 0xB0; //
        pub const m_bServerPaused: usize = 0x9E08; //
        pub const m_iPauseTeam: usize = 0x9E0C; //
        pub const m_nMatchClockUpdateTick: usize = 0x9E10; //
        pub const m_flMatchClockAtLastUpdate: usize = 0x9E14; //
        pub const m_flPauseTime: usize = 0x9E18; //
        pub const m_pausingPlayerId: usize = 0x9E20; //
        pub const m_unpausingPlayerId: usize = 0x9E24; //
        pub const m_fPauseRawTime: usize = 0x9E28; //
        pub const m_fPauseCurTime: usize = 0x9E2C; //
        pub const m_fUnpauseRawTime: usize = 0x9E30; //
        pub const m_fUnpauseCurTime: usize = 0x9E34; //
        pub const m_bRequiresReportCardDismissal: usize = 0x9E88; //
        pub const m_flPreGameWaitEndTime: usize = 0x9E8C; //
        pub const m_flReportCardDismissalWaitStart: usize = 0x9E90; //
        pub const m_nLastPreGameCount: usize = 0x9E94; //
        pub const m_eGGTeam: usize = 0x9E98; //
        pub const m_flGGEndsAtTime: usize = 0x9E9C; //
        pub const m_unMatchID: usize = 0x9EA0; //
        pub const m_sGameplayExperiment: usize = 0x9EA8; //
        pub const m_nPlayerDeathEventID: usize = 0x9EB0; //
        pub const m_nReplayChangedEvent: usize = 0x9EB4; //
        pub const m_nGameOverEvent: usize = 0x9EB8; //
        pub const m_flHeroDiedTime: usize = 0x9EE0; //
    }
    // Parent: CCitadelModifierAura
    // Field count: 0
    pub mod CCitadel_Modifier_Tier2Boss_RocketDamage_Aura {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CCitadel_Modifier_Rutger_Pulse_VData {
        pub const m_strSilenceTargetSound: usize = 0x638; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CAbilityShivDashVData {
        pub const m_DashModifier: usize = 0x1590; //
        pub const m_DashImpactEffect: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DashSwingEffect: usize = 0x1680; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_DashLineEffect: usize = 0x1760; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strDashStartWithTargets: usize = 0x1840; //
        pub const m_strDashStartEcho: usize = 0x1850; //
        pub const m_strDashStartMiss: usize = 0x1860; //
        pub const m_strDashHitEnemy: usize = 0x1870; //
        pub const m_flEchoDelay: usize = 0x1880; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Nano_Pounce_Self {
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 0
    pub mod CCitadel_Ability_MobileResupply {
    }
    // Parent: CitadelAbilityVData
    // Field count: 14
    pub mod CAbilityHornetSnipeVData {
        pub const m_AssassinateShotParticle: usize = 0x1590; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_AssassinateShotParticleOwnerOnly: usize = 0x1670; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LaserSightParticle: usize = 0x1750; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_LaserSightParticleOwnerOnly: usize = 0x1830; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_SnipeModifier: usize = 0x1910; //
        pub const m_BuffOnKillModifier: usize = 0x1920; //
        pub const m_GlowEnemyModifier: usize = 0x1930; //
        pub const m_strSnipeImpactSound: usize = 0x1940; //
        pub const m_strZoomIn: usize = 0x1950; //
        pub const m_strZoomOut: usize = 0x1960; //
        pub const m_flMinScopeTimeToShoot: usize = 0x1970; //
        pub const m_flFadeToBlackTime: usize = 0x1974; //
        pub const m_flFoVChangeTime: usize = 0x1978; //
        pub const m_ScopeFoV: usize = 0x1980; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_AblativeCoatResistBuff {
    }
    // Parent: CCitadelModifierVData
    // Field count: 1
    pub mod CModifierVitalitySuppressorVData {
        pub const m_DebuffParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: C_AI_CitadelNPC
    // Field count: 0
    pub mod C_CitadelPlayerBotNPCBrain {
    }
    // Parent: CBaseAnimGraph
    // Field count: 8
    pub mod C_RagdollProp {
        pub const m_ragPos: usize = 0xB00; //
        pub const m_ragAngles: usize = 0xB18; //
        pub const m_flBlendWeight: usize = 0xB30; //
        pub const m_hRagdollSource: usize = 0xB34; //
        pub const m_iEyeAttachment: usize = 0xB38; //
        pub const m_flBlendWeightCurrent: usize = 0xB3C; //
        pub const m_parentPhysicsBoneIndices: usize = 0xB40; //
        pub const m_worldSpaceBoneComputationOrder: usize = 0xB58; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_WeaponUpgrade_BansheeSlugs {
    }
    // Parent: CitadelAbilityVData
    // Field count: 9
    pub mod CBaseDashCastAbilityVData {
        pub const m_AbilityToTrigger: usize = 0x1590; //
        pub const m_flDashCastTriggerRadius: usize = 0x15A0; //
        pub const m_flDashSpeed: usize = 0x15A4; //
        pub const m_bSnapToZeroSpeedOnEnd: usize = 0x15A8; //
        pub const m_bUseCurveToDefineSpeed: usize = 0x15A9; //
        pub const m_MovementSpeedCurve: usize = 0x15B0; //
        pub const m_flMovementSpeedCurveAvgSpeed: usize = 0x15F0; //
        pub const m_strTargetHitSound: usize = 0x15F8; //
        pub const m_strMissSound: usize = 0x1608; //
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CModifier_Mirage_Traveler_MovementSpeed {
    }
    // Parent: CCitadelModifierVData
    // Field count: 0
    pub mod CModifier_Synth_Barrage_Amp_VData {
    }
    // Parent: CitadelAbilityVData
    // Field count: 4
    pub mod CCitadel_Ability_Gravity_Lasso_VData {
        pub const m_GravityLassoSelf: usize = 0x1590; //
        pub const m_GravityLassoTarget: usize = 0x15A0; //
        pub const m_TargetWarningSound: usize = 0x15B0; //
        pub const m_PreCastParticle: usize = 0x15C0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadel_Modifier_BaseEventProcVData
    // Field count: 4
    pub mod CCitadel_Modifier_MeleeCharge_VData {
        pub const m_SwingParticle: usize = 0x668; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HitParticle: usize = 0x748; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ReloadVisualModifier: usize = 0x828; //
        pub const m_AmmoAddedVisualModifier: usize = 0x838; //
    }
    // Parent: CitadelItemVData
    // Field count: 3
    pub mod CCitadel_Item_HealthRegenAuraVData {
        pub const m_HealParticle: usize = 0x15D8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_CastHealParticle: usize = 0x16B8; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_HealingPulseTrackerModifier: usize = 0x1798; //
    }
    // Parent: CCitadel_Modifier_BaseShield
    // Field count: 0
    pub mod CCitadel_Modifier_GalvanicStormTechShield {
    }
    // Parent: CCitadel_Modifier_RegeneratingBulletShield
    // Field count: 0
    pub mod CCitadel_Modifier_TempBulletShield {
    }
    // Parent: CCitadelModifier
    // Field count: 0
    pub mod CCitadel_Modifier_Root {
    }
    // Parent: C_BaseEntity
    // Field count: 1
    pub mod C_HeroPreview {
        pub const m_CCitadelHeroComponent: usize = 0x560; //
    }
    // Parent: None
    // Field count: 3
    pub mod ViewAngleServerChange_t {
        pub const nType: usize = 0x30; //
        pub const qAngle: usize = 0x34; //
        pub const nIndex: usize = 0x40; //
    }
    // Parent: CCitadel_Item
    // Field count: 0
    pub mod CCitadel_ArmorUpgrade_Colossus {
    }
    // Parent: CCitadel_Ability_BaseHeldItemVData
    // Field count: 15
    pub mod CCitadel_Ability_GoldenIdolVData {
        pub const m_sIdolDropOffSound: usize = 0x1670; //
        pub const m_DropoffTimerModifier: usize = 0x1680; //
        pub const m_HoldingIdolModifier: usize = 0x1690; //
        pub const m_RevealedHoldingIdolModifier: usize = 0x16A0; //
        pub const m_Bonus01: usize = 0x16B0; //
        pub const m_Bonus02: usize = 0x16C0; //
        pub const m_NoPickupModifier: usize = 0x16D0; //
        pub const m_flInstantGoldPercentage: usize = 0x16E0; //
        pub const m_iComebackBounty: usize = 0x16E4; //
        pub const m_flCasterBonusPercent: usize = 0x16E8; //
        pub const m_flRevealTime: usize = 0x16EC; //
        pub const m_flDamageTickRate: usize = 0x16F0; //
        pub const m_flMaxHealthDamage: usize = 0x16F4; //
        pub const m_flTimeToDamage: usize = 0x16F8; //
        pub const m_flNoPickupTime: usize = 0x16FC; //
    }
    // Parent: CitadelAbilityVData
    // Field count: 2
    pub mod CCitadel_Ability_Magician_ShadowCloneVData {
        pub const m_CloneModifier: usize = 0x1590; //
        pub const m_ExplodeParticle: usize = 0x15A0; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
    }
    // Parent: CCitadelModifier
    // Field count: 1
    pub mod CCitadel_Modifier_Thumper_Ability_2 {
        pub const m_vLastPosition: usize = 0xC0; //
    }
    // Parent: CCitadel_Modifier_BaseBulletPreRollProcVData
    // Field count: 3
    pub mod CCitadel_Modifier_BulletFlurryVData {
        pub const m_ImpactParticle: usize = 0x768; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_ExplodeParticle: usize = 0x848; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_strAttackerHitSound: usize = 0x928; //
    }
    // Parent: C_CitadelBaseAbility
    // Field count: 1
    pub mod CCitadel_Ability_CardToss {
        pub const m_bCardIsFlying: usize = 0xF20; //
    }
    // Parent: CCitadelModifierVData
    // Field count: 5
    pub mod CCitadel_Modifier_TeleportToObjectiveVData {
        pub const m_TeleportOriginParticle: usize = 0x638; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportDestinationParticle: usize = 0x718; // CResourceNameTyped<CWeakHandle<InfoForResourceTypeIParticleSystemDefinition>>
        pub const m_TeleportStartSound: usize = 0x7F8; //
        pub const m_TeleportCompleteSound: usize = 0x808; //
        pub const m_TeleportArriveSound: usize = 0x818; //
    }
    // Parent: None
    // Field count: 3
    pub mod StatViewerModifierValues_t {
        pub const m_SourceModifierID: usize = 0x30; //
        pub const m_eValType: usize = 0x34; //
        pub const m_flValue: usize = 0x38; //
    }
    // Parent: None
    // Field count: 2
    pub mod PhysicsRagdollPose_t {
        pub const m_Transforms: usize = 0x8; //
        pub const m_hOwner: usize = 0x20; //
    }
}
