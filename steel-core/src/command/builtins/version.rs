//! Vanilla server-version reporting command.

use steel_registry::packets::CURRENT_MC_PROTOCOL;
use steel_utils::{Identifier, MC_VERSION, translations};
use text_components::TextComponent;

use super::super::{
    brigadier::{CommandNodeBuilder, CommandSyntaxError},
    execution::{CommandSource, SteelCommandContext, SteelCommandRuntime, literal},
    registration::CommandRegistration,
};

pub(super) fn registration() -> CommandRegistration<CommandSource> {
    CommandRegistration::new(Identifier::vanilla_static("version"), |_| command())
}

fn command() -> CommandNodeBuilder<CommandSource, SteelCommandRuntime> {
    literal("version").executes(send_version)
}

/// Reports the id, name, protocol, and stability of the targeted vanilla
/// version. Vanilla's `/version` also reports a save-data version, a
/// save-data version series, a build time, and resource/data pack versions -
/// Steel doesn't track any of those yet, so those lines are left out rather
/// than filled in with made-up numbers.
#[expect(
    clippy::unnecessary_wraps,
    reason = "Command executors use a shared fallible callback signature."
)]
fn send_version(context: &SteelCommandContext<CommandSource>) -> Result<i32, CommandSyntaxError> {
    let source = context.source();
    source.send_system_message(&TextComponent::from(&translations::COMMANDS_VERSION_HEADER));
    source.send_system_message(
        &translations::COMMANDS_VERSION_ID
            .message([MC_VERSION])
            .component(),
    );
    source.send_system_message(
        &translations::COMMANDS_VERSION_NAME
            .message([MC_VERSION])
            .component(),
    );
    source.send_system_message(
        &translations::COMMANDS_VERSION_PROTOCOL
            .message([
                CURRENT_MC_PROTOCOL.to_string(),
                format!("0x{CURRENT_MC_PROTOCOL:x}"),
            ])
            .component(),
    );
    // Steel only ever targets full releases of MC_VERSION, never a
    // pre-release build of it (e.g. "26.3-rc-1").
    let stable = if MC_VERSION.contains('-') {
        &translations::COMMANDS_VERSION_STABLE_NO
    } else {
        &translations::COMMANDS_VERSION_STABLE_YES
    };
    source.send_system_message(&TextComponent::from(stable));
    Ok(1)
}

#[cfg(test)]
mod tests {
    use steel_registry::init_vanilla_registry;

    use super::super::create_dispatcher;

    #[test]
    fn version_is_a_bare_executable_literal() {
        init_vanilla_registry();
        let Ok(dispatcher) = create_dispatcher() else {
            panic!("built-in commands should register");
        };
        let Some(version) = dispatcher.children(dispatcher.root()).and_then(|children| {
            children.iter().copied().find(|child| {
                dispatcher
                    .node(*child)
                    .is_some_and(|node| node.name() == "version")
            })
        }) else {
            panic!("version root should exist");
        };
        let Some(node) = dispatcher.node(version) else {
            panic!("version root node should exist");
        };
        assert!(node.is_executable());
        assert!(node.argument_type().is_none());
    }
}
