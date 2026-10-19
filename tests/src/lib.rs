// Copyright 2019 PingCAP, Inc.

#[cfg(feature = "grpcio-prost-codec")]
extern crate prost_legacy as prost;

#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(renamed_and_removed_lints)]
#[allow(bare_trait_objects)]
#[allow(unused_parens)]
#[allow(mismatched_lifetime_syntaxes)]

mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

#[cfg(test)]
mod tests {
    use super::protos::compat::Compat;
    #[cfg(any(feature = "prost-codec", feature = "grpcio-prost-codec"))]
    use super::protos::nested::event::{Entries, Row};
    #[cfg(feature = "protobuf-codec")]
    use super::protos::nested::{Event_Entries as Entries, Event_Row as Row};
    use protobuf::Message;

    #[test]
    fn nested_messages_round_trip() {
        let mut row = Row::default();
        row.set_op_type(Default::default());
        assert_eq!(row.get_op_type(), Default::default());

        let mut entries = Entries::default();
        entries.set_entries(vec![row].into());
        let bytes = entries.write_to_bytes().unwrap();
        let mut decoded = Entries::default();
        decoded.merge_from_bytes(&bytes).unwrap();
        assert_eq!(entries, decoded);
        assert_eq!(decoded.get_entries().len(), 1);
    }

    #[test]
    fn optional_fields_and_enum_accessors() {
        let mut message = Compat::default();
        assert!(!message.has_kind());
        assert_eq!(message.get_kind() as i32, 0);
        // Set the enum to ACTIVE through its wire representation.
        message.merge_from_bytes(&[8, 1]).unwrap();
        assert!(message.has_kind());
        assert_eq!(message.get_kind() as i32, 1);
        let kind = message.get_kind();
        message.clear_kind();
        assert!(!message.has_kind());
        message.set_kind(kind);

        message.set_payload(vec![1, 2, 3].into());
        assert_eq!(message.get_payload(), &[1, 2, 3]);
        assert_eq!(message.take_payload(), vec![1, 2, 3]);
        assert!(!message.has_payload());

        let bytes = message.write_to_bytes().unwrap();
        let mut decoded = Compat::default();
        decoded.merge_from_bytes(&bytes).unwrap();
        assert_eq!(decoded, message);
    }
}
