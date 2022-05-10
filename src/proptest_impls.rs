// Copyright (c) The camino Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! [proptest::Arbitrary](Arbitrary) implementation for `Utf8PathBuf` and `Box<Utf8Path>`.  Note
//! that implementions for `Rc<Utc8Path>` and `Arc<Utf8Path>` are not currently possible due to
//! orphan rules - this crate doesn't define `Rc`/`Arc` nor `Arbitrary`, so it can't define those
//! implementations.

use proptest::{
    arbitrary::{any_with, Arbitrary, StrategyFor},
    strategy::{MapInto, Strategy},
};

use crate::{Utf8Path, Utf8PathBuf};

impl Arbitrary for Utf8PathBuf {
    type Parameters = <String as Arbitrary>::Parameters;
    type Strategy = MapInto<StrategyFor<String>, Self>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        any_with::<String>(args).prop_map_into()
    }
}

impl Arbitrary for Box<Utf8Path> {
    type Parameters = <Utf8PathBuf as Arbitrary>::Parameters;
    type Strategy = MapInto<StrategyFor<Utf8PathBuf>, Self>;
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        any_with::<Utf8PathBuf>(args).prop_map_into()
    }
}
