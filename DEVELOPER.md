## Developer Guide

A quick guide to some project-specific properties of this repo.

### Rich Docs

The project is set-up so that you can generate rich docs with support for 
mermaid diagrams and KaTeX with `cargo make rich-doc`. To use KaTeX, wrap
your math in $ ... $ or $$ ... $$ for block expressions. For mermaid, just
use a code block with language set to mermaid.

### `shields_up` Feature

There is a special feature which you can use to enable additional pre/post
condition checks. It is on by default, so make sure to disable it when
running benchmarks or public releases. Internal releases should probably
have `shields_up` enabled since they are often used to test and expose problems.

Remember that performance impact of `shields_up` does not have to be very
big - depends on the task. But it can also be significant, so if the code is 
taking a really long time to run, you can also try a build where the feature 
is disabled (assuming you trust the code).

### Code Coverage and CI

The project is configured for Travis with tarpaulin code coverage measurement.
Tarpaulin is not cross platform so you probably can't use it locally unless 
you are running on linux (even then there are some caveats). The best way
to test coverage is usually to actually push the code. Note that cargo on 
travis is cached because it takes forever to compile tarpaulin - if
something breaks, maybe it just needs updating - delete cache. 

### Conventions

I always try to document private items because in scientific code, these
are usually the most complex ones. Also, implementation details are often
part of official docs because they are often needed to understand real
complexity of algorithms, etc. In general, I prefer longer, self-contained
explanations for each main component (e.g. a struct) with short
references to the main text for sub-components (e.g. a method).

TL;DR: If your method documentation is more than one paragraph, you should probably
move it to struct. For function, move to module. Also, struct should not
talk about other structures - have a module documentation for that.

Also, I despise navigating rust projects because I never know what is defined 
where based on file names. Therefore I generally tend to create one module/file 
for each public struct and also for non-trivial private structures (similar 
for larger function groups, like file export/import). In general, it should be 
immediately clear from the name of the function/struct in which file it can be found
and from the file name what functions/structures are in it. However, this often 
introduces need for re-exporting :( In that case, I often prefer separating 
private aspects of implementation to keep public parts as minimal as possible
so they can actually fit into a single file easily.