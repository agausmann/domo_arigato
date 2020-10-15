use crate::nbt::Nbt;
use crate::proto::types::*;
use crate::util::{Greedy, LengthPrefix};
use declio::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Clientbound {
    #[declio(id = "VarInt(0x00)")]
    SpawnEntity {
        entity_id: VarInt,
        object_uuid: Uuid,
        type_: VarInt,
        x: Double,
        y: Double,
        z: Double,
        pitch: Angle,
        yaw: Angle,
        data: Int,
        velocity_x: Short,
        velocity_y: Short,
        velocity_z: Short,
    },

    #[declio(id = "VarInt(0x01)")]
    SpawnExperienceOrb {
        entity_id: VarInt,
        x: Double,
        y: Double,
        z: Double,
        count: Short,
    },

    #[declio(id = "VarInt(0x02)")]
    SpawnLivingEntity {
        entity_id: VarInt,
        entity_uuid: Uuid,
        type_: VarInt,
        x: Double,
        y: Double,
        z: Double,
        yaw: Angle,
        pitch: Angle,
        head_pitch: Angle,
        velocity_x: Short,
        velocity_y: Short,
        velocity_z: Short,
    },

    #[declio(id = "VarInt(0x03)")]
    SpawnPainting {
        entity_id: VarInt,
        entity_uuid: Uuid,
        motive: VarInt,
        location: Position,
        direction: Byte,
    },

    #[declio(id = "VarInt(0x04)")]
    SpawnPlayer {
        entity_id: VarInt,
        player_uuid: Uuid,
        x: Double,
        y: Double,
        z: Double,
        yaw: Angle,
        pitch: Angle,
    },

    #[declio(id = "VarInt(0x05)")]
    EntityAnimation { entity_id: VarInt, animation: UByte },

    #[declio(id = "VarInt(0x06)")]
    Statistics {
        #[declio(with = "LengthPrefix::<VarInt>")]
        statistics: Vec<Statistic>,
    },

    #[declio(id = "VarInt(0x07)")]
    AcknowledgePlayerDigging {
        location: Position,
        block: VarInt,
        status: VarInt,
        success: Boolean,
    },

    #[declio(id = "VarInt(0x08)")]
    BlockBreakAnimation {
        entity_id: VarInt,
        location: Position,
        destroy_stage: Byte,
    },

    #[declio(id = "VarInt(0x09)")]
    BlockEntityData {
        location: Position,
        action: UByte,
        nbt_data: Nbt,
    },

    #[declio(id = "VarInt(0x0a)")]
    BlockAction {
        location: Position,
        action_id: UByte,
        action_param: UByte,
        block_type: VarInt,
    },

    #[declio(id = "VarInt(0x0b)")]
    BlockChange {
        location: Position,
        block_id: VarInt,
    },

    #[declio(id = "VarInt(0x0c)")]
    BossBar { uuid: Uuid, action: BossBarAction },

    #[declio(id = "VarInt(0x0d)")]
    ServerDifficulty { difficulty: UByte, locked: bool },

    #[declio(id = "VarInt(0x0e)")]
    ChatMessage {
        json_data: Chat,
        position: Byte,
        sender: Uuid,
    },

    #[declio(id = "VarInt(0x0f)")]
    TabComplete {
        id: VarInt,
        start: VarInt,
        length: VarInt,
        #[declio(with = "LengthPrefix::<VarInt>")]
        matches: Vec<TabCompleteMatch>,
    },

    #[declio(id = "VarInt(0x10)")]
    DeclareCommands {
        #[declio(with = "Greedy")]
        todo: ByteArray,
    },

    #[declio(id = "VarInt(0x11)")]
    WindowConfirmation {
        window_id: UByte,
        action_number: Short,
        accepted: Boolean,
    },

    #[declio(id = "VarInt(0x12)")]
    CloseWindow { window_id: UByte },

    #[declio(id = "VarInt(0x13)")]
    WindowItems {
        window_id: UByte,
        #[declio(with = "LengthPrefix::<Short>")]
        slot_data: Vec<Slot>,
    },

    #[declio(id = "VarInt(0x14)")]
    WindowProperty {
        window_id: UByte,
        property: Short,
        value: Short,
    },

    #[declio(id = "VarInt(0x15)")]
    SetSlot {
        window_id: UByte,
        slot: Short,
        slot_data: Slot,
    },

    #[declio(id = "VarInt(0x16)")]
    SetCooldown {
        item_id: VarInt,
        cooldown_ticks: VarInt,
    },

    #[declio(id = "VarInt(0x17)")]
    PluginMessage {
        channel: Identifier,
        #[declio(with = "Greedy")]
        data: ByteArray,
    },

    #[declio(id = "VarInt(0x18)")]
    NamedSoundEffect {
        sound_name: Identifier,
        sound_category: VarInt,
        effect_position_x: Int,
        effect_position_y: Int,
        effect_position_z: Int,
        volume: Float,
        pitch: Float,
    },

    #[declio(id = "VarInt(0x19)")]
    Disconnect { reason: Chat },

    #[declio(id = "VarInt(0x1a)")]
    EntityStatus { entity_id: Int, entity_status: Byte },

    #[declio(id = "VarInt(0x1b)")]
    Explosion {
        x: Float,
        y: Float,
        z: Float,
        strength: Float,
        #[declio(with = "LengthPrefix::<Int>")]
        records: Vec<ExplosionRecord>,
        player_motion_x: Float,
        player_motion_y: Float,
        player_motion_z: Float,
    },

    #[declio(id = "VarInt(0x1c)")]
    UnloadChunk { chunk_x: Int, chunk_z: Int },

    #[declio(id = "VarInt(0x1d)")]
    ChangeGameState { reason: UByte, value: Float },

    #[declio(id = "VarInt(0x1e)")]
    OpenHorseWindow {
        window_id: UByte,
        slots: VarInt,
        entity_id: Int,
    },

    #[declio(id = "VarInt(0x1f)")]
    KeepAlive { keepalive_id: Long },

    #[declio(id = "VarInt(0x20)")]
    ChunkData {
        chunk_x: Int,
        chunk_z: Int,
        full_chunk: Boolean,
        primary_bit_mask: VarInt,
        heightmaps: Nbt,
        #[declio(skip_if = "!full_chunk", with = "LengthPrefix::<VarInt>")]
        biomes: Vec<VarInt>,
        #[declio(with = "LengthPrefix::<VarInt>")]
        data: ByteArray,
        #[declio(with = "LengthPrefix::<VarInt>")]
        block_entities: Vec<Nbt>,
    },

    #[declio(id = "VarInt(0x21)")]
    Effect {
        effect_id: Int,
        location: Position,
        data: Int,
        disable_relative_volume: Boolean,
    },

    #[declio(id = "VarInt(0x22)")]
    Particle {
        particle_id: Int,
        long_distance: Boolean,
        x: Double,
        y: Double,
        z: Double,
        offset_x: Float,
        offset_y: Float,
        offset_z: Float,
        particle_data: Float,
        particle_count: Int,
        #[declio(with = "Greedy")]
        todo_data: ByteArray,
    },

    #[declio(id = "VarInt(0x23)")]
    UpdateLight {
        chunk_x: VarInt,
        chunk_z: VarInt,
        trust_edges: Boolean,
        sky_light_mask: VarInt,
        block_light_mask: VarInt,
        empty_sky_light_mask: VarInt,
        empty_block_light_mask: VarInt,
        #[declio(with = "Greedy")]
        todo_light_arrays: ByteArray,
    },

    #[declio(id = "VarInt(0x24)")]
    JoinGame {
        entity_id: Int,
        is_hardcore: Boolean,
        gamemode: Gamemode,
        previous_gamemode: UByte,
        #[declio(with = "LengthPrefix::<VarInt>")]
        worlds: Vec<Identifier>,
        dimension_codec: Nbt,
        dimension: Nbt,
        world_name: Identifier,
        seed_hash: Long,
        max_players: VarInt,
        view_distance: VarInt,
        reduced_debug_info: Boolean,
        enable_respawn_screen: Boolean,
        is_debug: Boolean,
        is_flat: Boolean,
    },

    #[declio(id = "VarInt(0x25)")]
    MapData {
        map_id: VarInt,
        scale: Byte,
        tracking_position: Boolean,
        locked: Boolean,
        #[declio(with = "LengthPrefix::<VarInt>")]
        icons: Vec<MapIcon>,
        columns: UByte,
        #[declio(skip_if = "*columns == 0")]
        rows: Option<UByte>,
        #[declio(skip_if = "*columns == 0")]
        x: Option<UByte>,
        #[declio(skip_if = "*columns == 0")]
        z: Option<UByte>,
        #[declio(skip_if = "*columns == 0", with = "LengthPrefix::<VarInt>")]
        data: Vec<u8>,
    },

    #[declio(id = "VarInt(0x26)")]
    TradeList {
        window_id: VarInt,
        #[declio(with = "LengthPrefix::<Byte>")]
        trades: Vec<Trade>,
        villager_level: VarInt,
        experience: VarInt,
        is_regular_villager: Boolean,
        can_restock: Boolean,
    },

    #[declio(id = "VarInt(0x27)")]
    EntityPosition {
        entity_id: VarInt,
        delta_x: Short,
        delta_y: Short,
        delta_z: Short,
        on_ground: Boolean,
    },

    #[declio(id = "VarInt(0x28)")]
    EntityPositionAndRotation {
        entity_id: VarInt,
        delta_x: Short,
        delta_y: Short,
        delta_z: Short,
        yaw: Angle,
        pitch: Angle,
        on_ground: Boolean,
    },

    #[declio(id = "VarInt(0x29)")]
    EntityRotation {
        entity_id: VarInt,
        yaw: Angle,
        pitch: Angle,
        on_ground: Boolean,
    },

    #[declio(id = "VarInt(0x2a)")]
    EntityMovement { entity_id: VarInt },

    #[declio(id = "VarInt(0x2b)")]
    VehicleMove {
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
    },

    #[declio(id = "VarInt(0x2c)")]
    OpenBook { hand: VarInt },

    #[declio(id = "VarInt(0x2d)")]
    OpenWindow {
        window_id: VarInt,
        window_type: VarInt,
        window_title: Chat,
    },

    #[declio(id = "VarInt(0x2e)")]
    OpenSignEditor { location: Position },

    #[declio(id = "VarInt(0x2f)")]
    CraftRecipeResponse {
        window_id: UByte,
        recipe: Identifier,
    },

    #[declio(id = "VarInt(0x30)")]
    PlayerAbilities {
        flags: Byte,
        flying_speed: Float,
        field_of_view: Float,
    },

    #[declio(id = "VarInt(0x31)")]
    CombatEvent { event: CombatEvent },

    #[declio(id = "VarInt(0x32)")]
    PlayerInfo { action: PlayerInfoAction },

    #[declio(id = "VarInt(0x33)")]
    FacePlayer {
        origin: VarInt,
        target_x: Double,
        target_y: Double,
        target_z: Double,
        is_entity: Boolean,
        #[declio(skip_if = "!is_entity")]
        entity_id: Option<VarInt>,
        #[declio(skip_if = "!is_entity")]
        entity_origin: Option<VarInt>,
    },

    #[declio(id = "VarInt(0x34)")]
    PlayerPositionAndLook {
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        flags: Byte,
        teleport_id: VarInt,
    },

    #[declio(id = "VarInt(0x35)")]
    UnlockRecipes {
        action: VarInt,
        crafting_recipe_book_open: Boolean,
        crafting_recipe_book_filter_active: Boolean,
        smelting_recipe_book_open: Boolean,
        smelting_recipe_book_filter_active: Boolean,
        blast_furnace_recipe_book_open: Boolean,
        blast_furnace_recipe_book_filter_active: Boolean,
        smoker_recipe_book_open: Boolean,
        smoker_recipe_book_filter_active: Boolean,
        #[declio(with = "LengthPrefix::<VarInt>")]
        recipe_list_1: Vec<Identifier>,
        #[declio(skip_if = "*action != VarInt(0)", with = "LengthPrefix::<VarInt>")]
        recipe_list_2: Vec<Identifier>,
    },

    #[declio(id = "VarInt(0x36)")]
    DestroyEntities {
        #[declio(with = "LengthPrefix::<VarInt>")]
        entity_ids: Vec<VarInt>,
    },

    #[declio(id = "VarInt(0x37)")]
    RemoveEntityEffect { entity_id: VarInt, effect_id: Byte },

    #[declio(id = "VarInt(0x38)")]
    ResourcePackSend { url: String, sha1_hash: String },

    #[declio(id = "VarInt(0x39)")]
    Respawn {
        dimension: Nbt,
        world_name: Identifier,
        seed_hash: Long,
        gamemode: UByte,
        previous_gamemode: UByte,
        is_debug: Boolean,
        is_flat: Boolean,
        copy_metadata: Boolean,
    },

    #[declio(id = "VarInt(0x3a)")]
    EntityHeadLook { entity_id: VarInt, head_yaw: Angle },

    #[declio(id = "VarInt(0x3b)")]
    MultiBlockChange {
        chunk_section: Long,
        no_trust_edges: Boolean,
        #[declio(with = "LengthPrefix::<VarInt>")]
        records: Vec<VarLong>,
    },

    #[declio(id = "VarInt(0x3c)")]
    SelectAdvancementTab {
        has_id: Boolean,
        #[declio(skip_if = "!has_id")]
        id: Option<String>,
    },

    #[declio(id = "VarInt(0x3d)")]
    WorldBorder { action: WorldBorderAction },

    #[declio(id = "VarInt(0x3e)")]
    Camera { camera_id: VarInt },

    #[declio(id = "VarInt(0x3f)")]
    HeldItemChange { slot: Byte },

    #[declio(id = "VarInt(0x40)")]
    UpdateViewPosition { chunk_x: VarInt, chunk_z: VarInt },

    #[declio(id = "VarInt(0x41)")]
    UpdateViewDistance { view_distance: VarInt },

    #[declio(id = "VarInt(0x42)")]
    SpawnPosition { location: Position },

    #[declio(id = "VarInt(0x43)")]
    DisplayScoreboard { position: Byte, name: String },

    #[declio(id = "VarInt(0x44)")]
    EntityMetadata {
        entity_id: VarInt,
        #[declio(with = "Greedy")]
        todo_metadata: ByteArray,
    },

    #[declio(id = "VarInt(0x45)")]
    AttachEntity {
        attached_entity_id: Int,
        holding_entity_id: Int,
    },

    #[declio(id = "VarInt(0x46)")]
    EntityVelocity {
        entity_id: VarInt,
        velocity_x: Short,
        velocity_y: Short,
        velocity_z: Short,
    },

    #[declio(id = "VarInt(0x47)")]
    EntityEquipment {
        entity_id: VarInt,
        #[declio(with = "Greedy")]
        todo: ByteArray,
    },

    #[declio(id = "VarInt(0x48)")]
    SetExperience {
        experience_bar: Float,
        level: VarInt,
        total_experience: VarInt,
    },

    #[declio(id = "VarInt(0x49)")]
    UpdateHealth {
        health: Float,
        food: VarInt,
        food_saturation: Float,
    },

    #[declio(id = "VarInt(0x4a)")]
    ScoreboardObjective {
        objective_name: String,
        mode: Byte,
        #[declio(skip_if = "*mode == 1")]
        objective_value: Option<Chat>,
        #[declio(skip_if = "*mode == 1")]
        type_: Option<VarInt>,
    },

    #[declio(id = "VarInt(0x4b)")]
    SetPassengers {
        entity_id: VarInt,
        #[declio(with = "LengthPrefix::<VarInt>")]
        passengers: Vec<VarInt>,
    },

    #[declio(id = "VarInt(0x4c)")]
    Teams {
        team_name: String,
        action: TeamsAction,
    },

    #[declio(id = "VarInt(0x4d)")]
    UpdateScore {
        entity_name: String,
        action: Byte,
        objective_name: String,
        #[declio(skip_if = "*action == 1")]
        value: Option<VarInt>,
    },

    #[declio(id = "VarInt(0x4e)")]
    TimeUpdate { world_age: Long, time_of_day: Long },

    #[declio(id = "VarInt(0x4f)")]
    Title { action: TitleAction },

    #[declio(id = "VarInt(0x50)")]
    EntitySoundEffect {
        sound_id: VarInt,
        sound_category: VarInt,
        entity_id: VarInt,
        volume: Float,
        pitch: Float,
    },

    #[declio(id = "VarInt(0x51)")]
    SoundEffect {
        sound_id: VarInt,
        sound_category: VarInt,
        effect_position_x: Int,
        effect_position_y: Int,
        effect_position_z: Int,
        volume: Float,
        pitch: Float,
    },

    #[declio(id = "VarInt(0x52)")]
    StopSound {
        flags: Byte,
        #[declio(skip_if = "*flags & 0x1 == 0")]
        source: Option<VarInt>,
        #[declio(skip_if = "*flags & 0x2 == 0")]
        sound: Option<Identifier>,
    },

    #[declio(id = "VarInt(0x53)")]
    PlayerListHeaderAndFooter { header: Chat, footer: Chat },

    #[declio(id = "VarInt(0x54)")]
    NbtQueryResponse { transaction_id: VarInt, nbt: Nbt },

    #[declio(id = "VarInt(0x55)")]
    CollectItem {
        collected_entity_id: VarInt,
        collector_entity_id: VarInt,
        pickup_item_count: VarInt,
    },

    #[declio(id = "VarInt(0x56)")]
    EntityTeleport {
        entity_id: VarInt,
        x: Double,
        y: Double,
        z: Double,
        yaw: Angle,
        pitch: Angle,
        on_ground: Boolean,
    },

    #[declio(id = "VarInt(0x57)")]
    Advancements {
        #[declio(with = "Greedy")]
        todo: ByteArray,
    },

    #[declio(id = "VarInt(0x58)")]
    EntityProperties {
        #[declio(with = "Greedy")]
        todo: ByteArray,
    },

    #[declio(id = "VarInt(0x59)")]
    EntityEffect {
        entity_id: VarInt,
        effect_id: Byte,
        amplifier: Byte,
        duration: VarInt,
        flags: Byte,
    },

    #[declio(id = "VarInt(0x5a)")]
    DeclareRecipes {
        #[declio(with = "Greedy")]
        todo: ByteArray,
    },

    #[declio(id = "VarInt(0x5b)")]
    Tags {
        #[declio(with = "LengthPrefix::<VarInt>")]
        block_tags: Vec<Tag>,
        #[declio(with = "LengthPrefix::<VarInt>")]
        item_tags: Vec<Tag>,
        #[declio(with = "LengthPrefix::<VarInt>")]
        fluid_tags: Vec<Tag>,
        #[declio(with = "LengthPrefix::<VarInt>")]
        entity_tags: Vec<Tag>,
    },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum Serverbound {
    #[declio(id = "VarInt(0x00)")]
    TeleportConfirm { teleport_id: VarInt },

    #[declio(id = "VarInt(0x01)")]
    QueryBlockNbt {
        transaction_id: VarInt,
        location: Position,
    },

    #[declio(id = "VarInt(0x0d)")]
    QueryEntityNbt {
        transaction_id: VarInt,
        entity_id: VarInt,
    },

    #[declio(id = "VarInt(0x02)")]
    SetDifficulty { difficulty: Byte },

    #[declio(id = "VarInt(0x03)")]
    ChatMessage { message: String },

    #[declio(id = "VarInt(0x04)")]
    ClientStatus { action: VarInt },

    #[declio(id = "VarInt(0x05)")]
    ClientSettings {
        locale: String,
        view_distance: Byte,
        chat_mode: VarInt,
        chat_colors: Boolean,
        displayed_skin_parts: UByte,
        main_hand: VarInt,
    },

    #[declio(id = "VarInt(0x06)")]
    TabComplete {
        transaction_id: VarInt,
        text: String,
    },

    #[declio(id = "VarInt(0x07)")]
    WindowConfirmation {
        window_id: UByte,
        action_number: Short,
        accepted: Boolean,
    },

    #[declio(id = "VarInt(0x08)")]
    ClickWindowButton { window_id: UByte, button_id: Byte },

    #[declio(id = "VarInt(0x09)")]
    ClickWindow {
        window_id: UByte,
        slot: Short,
        button: Byte,
        action_number: Short,
        mode: VarInt,
        clicked: Slot,
    },

    #[declio(id = "VarInt(0x0a)")]
    CloseWindow { window_id: UByte },

    #[declio(id = "VarInt(0x0b)")]
    PluginMessage {
        channel: Identifier,
        #[declio(with = "Greedy")]
        data: ByteArray,
    },

    #[declio(id = "VarInt(0x0c)")]
    EditBook {
        book: Slot,
        is_signing: Boolean,
        hand: VarInt,
    },

    #[declio(id = "VarInt(0x0e)")]
    InteractEntity {
        entity_id: VarInt,
        action: InteractEntityAction,
        sneaking: Boolean,
    },

    #[declio(id = "VarInt(0x0f)")]
    GenerateStructure {
        location: Position,
        levels: VarInt,
        keep_jigsaws: Boolean,
    },

    #[declio(id = "VarInt(0x10)")]
    KeepAlive { keepalive_id: Long },

    #[declio(id = "VarInt(0x11)")]
    LockDifficulty { locked: Boolean },

    #[declio(id = "VarInt(0x12)")]
    PlayerPosition {
        x: Double,
        y: Double,
        z: Double,
        on_ground: Boolean,
    },

    #[declio(id = "VarInt(0x13)")]
    PlayerPositionAndRotation {
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
        on_ground: Boolean,
    },

    #[declio(id = "VarInt(0x14)")]
    PlayerRotation {
        yaw: Float,
        pitch: Float,
        on_ground: Boolean,
    },

    #[declio(id = "VarInt(0x15)")]
    PlayerMovement { on_ground: Boolean },

    #[declio(id = "VarInt(0x16)")]
    VehicleMove {
        x: Double,
        y: Double,
        z: Double,
        yaw: Float,
        pitch: Float,
    },

    #[declio(id = "VarInt(0x17)")]
    SteerBoat {
        left_paddle_turning: Boolean,
        right_paddle_turning: Boolean,
    },

    #[declio(id = "VarInt(0x18)")]
    PickItem { slot: VarInt },

    #[declio(id = "VarInt(0x19)")]
    CraftRecipeRequest {
        window_id: UByte,
        recipe: Identifier,
        make_all: Boolean,
    },

    #[declio(id = "VarInt(0x1a)")]
    PlayerAbilities { flags: Byte },

    #[declio(id = "VarInt(0x1b)")]
    PlayerDigging {
        status: VarInt,
        location: Position,
        face: Byte,
    },

    #[declio(id = "VarInt(0x1c)")]
    EntityAction {
        entity_id: VarInt,
        action_id: VarInt,
        jump_boost: VarInt,
    },

    #[declio(id = "VarInt(0x1d)")]
    SteerVehicle {
        sideways: Float,
        forward: Float,
        flags: UByte,
    },

    #[declio(id = "VarInt(0x1e)")]
    SetDisplayedRecipe { recipe_id: Identifier },

    #[declio(id = "VarInt(0x1f)")]
    SetRecipeBookState {
        book_id: VarInt,
        book_open: Boolean,
        filter_active: Boolean,
    },

    #[declio(id = "VarInt(0x20)")]
    NameItem { item_name: String },

    #[declio(id = "VarInt(0x21)")]
    ResourcePackStatus { result: VarInt },

    #[declio(id = "VarInt(0x22)")]
    AdvancementTab {
        action: VarInt,
        #[declio(skip_if = "*action != VarInt(0)")]
        tab_id: Option<Identifier>,
    },

    #[declio(id = "VarInt(0x23)")]
    SelectTrade { slot: VarInt },

    #[declio(id = "VarInt(0x24)")]
    SetBeaconEffect {
        primary_effect: VarInt,
        secondary_effect: VarInt,
    },

    #[declio(id = "VarInt(0x25)")]
    HeldItemChange { slot: Short },

    #[declio(id = "VarInt(0x26)")]
    UpdateCommandBlock {
        location: Position,
        command: String,
        mode: VarInt,
        flags: Byte,
    },

    #[declio(id = "VarInt(0x27)")]
    UpdateCommandBlockMinecart {
        entity_id: VarInt,
        command: String,
        track_output: Boolean,
    },

    #[declio(id = "VarInt(0x28)")]
    CreativeInventoryAction { slot: Short, clicked_item: Slot },

    #[declio(id = "VarInt(0x29)")]
    UpdateJigsawBlock {
        location: Position,
        name: Identifier,
        target: Identifier,
        pool: Identifier,
        final_state: String,
        joint_type: String,
    },

    #[declio(id = "VarInt(0x2a)")]
    UpdateStructureBlock {
        location: Position,
        action: VarInt,
        mode: VarInt,
        name: String,
        offset_x: Byte,
        offset_y: Byte,
        offset_z: Byte,
        size_x: Byte,
        size_y: Byte,
        size_z: Byte,
        mirror: VarInt,
        rotation: VarInt,
        metadata: String,
        integrity: Float,
        seed: VarLong,
        flags: Byte,
    },

    #[declio(id = "VarInt(0x2b)")]
    UpdateSign {
        location: Position,
        line_1: String,
        line_2: String,
        line_3: String,
        line_4: String,
    },

    #[declio(id = "VarInt(0x2c)")]
    Animation { hand: VarInt },

    #[declio(id = "VarInt(0x2d)")]
    Spectate { target_player: Uuid },

    #[declio(id = "VarInt(0x2e)")]
    PlayerBlockPlacement {
        hand: VarInt,
        location: Position,
        face: VarInt,
        cursor_position_x: Float,
        cursor_position_y: Float,
        cursor_position_z: Float,
        inside_block: Boolean,
    },

    #[declio(id = "VarInt(0x2f)")]
    UseItem { hand: VarInt },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Statistic {
    category_id: VarInt,
    statistic_id: VarInt,
    value: VarInt,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum BossBarAction {
    #[declio(id = "VarInt(0)")]
    Add {
        title: Chat,
        health: Float,
        color: VarInt,
        division: VarInt,
        flags: UByte,
    },

    #[declio(id = "VarInt(1)")]
    Remove,

    #[declio(id = "VarInt(2)")]
    UpdateHealth { health: Float },

    #[declio(id = "VarInt(3)")]
    UpdateTitle { title: Chat },

    #[declio(id = "VarInt(4)")]
    UpdateStyle {
        color: VarInt,
        division: VarInt,
        flags: UByte,
    },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct BlockChangeRecord {
    pub horizontal_position: UByte,
    pub y_coordinate: UByte,
    pub block_id: VarInt,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct TabCompleteMatch {
    pub match_: String,
    pub has_tooltip: Boolean,
    #[declio(skip_if = "!has_tooltip")]
    pub tooltip: Option<Chat>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct ExplosionRecord {
    pub x: Byte,
    pub y: Byte,
    pub z: Byte,
}

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
#[declio(id_type = "UByte")]
pub enum Gamemode {
    #[declio(id = "0")]
    Survival,

    #[declio(id = "1")]
    Creative,

    #[declio(id = "2")]
    Adventure,

    #[declio(id = "3")]
    Spectator,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct MapIcon {
    pub type_: VarInt,
    pub x: Byte,
    pub z: Byte,
    pub direction: Byte,
    pub has_display_name: Boolean,
    #[declio(skip_if = "!has_display_name")]
    pub display_name: Option<Chat>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Trade {
    pub input_item_1: Slot,
    pub output_item: Slot,
    pub has_second_item: Boolean,
    #[declio(skip_if = "!has_second_item")]
    pub input_item_2: Option<Slot>,
    pub trade_disabled: Boolean,
    pub trade_uses: Int,
    pub max_trade_uses: Int,
    pub xp: Int,
    pub special_price: Int,
    pub price_multipler: Float,
    pub demand: Int,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum CombatEvent {
    #[declio(id = "VarInt(0)")]
    EnterCombat,

    #[declio(id = "VarInt(1)")]
    EndCombat { duration: VarInt, entity_id: Int },

    #[declio(id = "VarInt(1)")]
    EntityDead {
        player_id: VarInt,
        entity_id: Int,
        message: Chat,
    },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum PlayerInfoAction {
    #[declio(id = "VarInt(0)")]
    AddPlayer {
        #[declio(with = "LengthPrefix::<VarInt>")]
        players: Vec<PlayerInfoAddPlayer>,
    },

    #[declio(id = "VarInt(1)")]
    UpdateGamemode {
        #[declio(with = "LengthPrefix::<VarInt>")]
        players: Vec<PlayerInfoUpdateGamemode>,
    },

    #[declio(id = "VarInt(2)")]
    UpdateLatency {
        #[declio(with = "LengthPrefix::<VarInt>")]
        players: Vec<PlayerInfoUpdateLatency>,
    },

    #[declio(id = "VarInt(3)")]
    UpdateDisplayName {
        #[declio(with = "LengthPrefix::<VarInt>")]
        players: Vec<PlayerInfoUpdateDisplayName>,
    },

    #[declio(id = "VarInt(4)")]
    RemovePlayer {
        #[declio(with = "LengthPrefix::<VarInt>")]
        players: Vec<PlayerInfoRemovePlayer>,
    },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct PlayerInfoAddPlayer {
    pub uuid: Uuid,
    pub name: String,
    #[declio(with = "LengthPrefix::<VarInt>")]
    pub properties: Vec<PlayerProperty>,
    pub gamemode: VarInt,
    pub ping: VarInt,
    pub has_display_name: Boolean,
    #[declio(skip_if = "!has_display_name")]
    pub display_name: Option<Chat>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub is_signed: Boolean,
    #[declio(skip_if = "!is_signed")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct PlayerInfoUpdateGamemode {
    pub uuid: Uuid,
    pub gamemode: VarInt,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct PlayerInfoUpdateLatency {
    pub uuid: Uuid,
    pub ping: VarInt,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct PlayerInfoUpdateDisplayName {
    pub uuid: Uuid,
    pub has_display_name: Boolean,
    #[declio(skip_if = "!has_display_name")]
    pub display_name: Option<Chat>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct PlayerInfoRemovePlayer {
    pub uuid: Uuid,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum WorldBorderAction {
    #[declio(id = "VarInt(0)")]
    SetSize { diameter: Double },

    #[declio(id = "VarInt(1)")]
    LerpSize {
        old_diameter: Double,
        new_diameter: Double,
        speed: VarLong,
    },

    #[declio(id = "VarInt(2)")]
    SetCenter { x: Double, z: Double },

    #[declio(id = "VarInt(3)")]
    Initialize {
        x: Double,
        z: Double,
        old_diameter: Double,
        new_diameter: Double,
        speed: VarLong,
        portal_teleport_boundary: VarInt,
        warning_time: VarInt,
        warning_blocks: VarInt,
    },

    #[declio(id = "VarInt(4)")]
    SetWarningTime { warning_time: VarInt },

    #[declio(id = "VarInt(5)")]
    SetWarningBlocks { warning_blocks: VarInt },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "Byte")]
pub enum TeamsAction {
    #[declio(id = "0")]
    CreateTeam {
        display_name: Chat,
        friendly_flags: Byte,
        name_tag_visibility: String,
        collision_rule: String,
        team_color: VarInt,
        team_prefix: Chat,
        team_suffix: Chat,
        #[declio(with = "LengthPrefix::<VarInt>")]
        entities: Vec<String>,
    },

    #[declio(id = "1")]
    RemoveTeam,

    #[declio(id = "2")]
    UpdateTeamInfo {
        team_display_name: Chat,
        friendly_flags: Byte,
        name_tag_visibility: String,
        collision_rule: String,
        team_color: VarInt,
        team_prefix: Chat,
        team_suffix: Chat,
    },

    #[declio(id = "3")]
    AddPlayers {
        #[declio(with = "LengthPrefix::<VarInt>")]
        entities: Vec<String>,
    },

    #[declio(id = "4")]
    RemovePlayers {
        #[declio(with = "LengthPrefix::<VarInt>")]
        entities: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum TitleAction {
    #[declio(id = "VarInt(0)")]
    SetTitle { text: Chat },

    #[declio(id = "VarInt(1)")]
    SetSubtitle { text: Chat },

    #[declio(id = "VarInt(2)")]
    SetActionBar { text: Chat },

    #[declio(id = "VarInt(3)")]
    SetTimesAndDisplay {
        fade_in: Int,
        stay: Int,
        fade_out: Int,
    },

    #[declio(id = "VarInt(4)")]
    Hide,

    #[declio(id = "VarInt(5)")]
    Reset,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Tag {
    pub name: Identifier,
    #[declio(with = "LengthPrefix::<VarInt>")]
    pub entries: Vec<VarInt>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[declio(id_type = "VarInt")]
pub enum InteractEntityAction {
    #[declio(id = "VarInt(0)")]
    Interact { hand: VarInt },

    #[declio(id = "VarInt(1)")]
    Attack,

    #[declio(id = "VarInt(2)")]
    InteractAt {
        target_x: Float,
        target_y: Float,
        target_z: Float,
        hand: VarInt,
    },
}
