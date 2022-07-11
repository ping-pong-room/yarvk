# Yarvk

**Y**et **A**nother **R**ust **V**ul**k**an is a hand-writing Vulkan API wrapper which build on top of Ash, an unsafe Vulkan API. Yarvk is
designed as similar as the C Vulkan API, while coding in a Rust way.

## Compare with other Rust Vulkan libraries:
1. compare to Ash: Yarvk build on ash and all APIs exposed are safe API.
2. compare to Vulkano: Yarvk do not hide Vuklan details in order to easy to use. For example
you still need to manage memories by yourself, and be careful of GPU side sychronization. I 
think this is good for programmer who is familiar with Vulkan C API, and want to start 
writing Rust codes as soon as possible.

## Safety
the concept safety is a little complicated here, I roughly divide them into two types: Rust 
level safety and Vulkan level safety.

1. Yarvk is totally Rust level safe, which means you won't get UB when dealing with Vulkan APIs,
for example no matter how to move a Vulkan struct type, the `p_next` always points to valid 
memories.

2. Yarvk also wrap Vulkan handles with lifetimes, so you are free from destroying Vulkan handles
you allocated before. Lifetime also helps to avoid to access invalid resources, for example, 
if there's any `Queue` exists, a `Device` shell never be destoryed. This is especially 
useful for command buffers, a common mistake of command buffers is that a CB is recorded 
with some vertex buffer commands, but the vertex buffer is freed by mistake before the CB is 
submitted.

3. Yarvk also guaranteed host side synchronization, thanks to the feature of Rust.

4. Type safety is guaranteed, such as you won't push wrong object to `p_next` by mistake.

As about Vulkan level safety (more offical name is Vulkan Validations):
1. One feature of Yarvk is that it guaranteed extension and feature safety, you won't
   use a feature without enabling the corresponded extension by mistake.
2. Some Validations are guaranteed, most of them are validations which have zero costs.

What Yarvk not guaranteed:
1. GPU side synchronization.
2. Validations that cannot be checked at compile time.

## Contributing
Yarvk needs you contributing. As mentioned above, every type is hand-wrote, because different
kind of Vulkan structs have different validations, which will lead to a lot of different 
syntax. There are still lots of Vulkan types which are not implemented yet, most of them
are commands and `p_next` extensions. If you do agree the design of this crate, please 
consider contribute this crate. _TODO: Contribute guide_.

## Stability
Yarvk for now is unstable because some of unstable features are used, that's why I also
didn't publish it to crate.io. All the features are const related features, 
and honestly speaking all of them can be workaround with stable features,
using them is just for convenient and code readability. Consider that const related features
are really active working progresses in Rust community,
my plan is to wait, until most of the Vulkan types are implemented, while there are still unstable
feature exists, I will refactor these unstable features to stable implementations. 