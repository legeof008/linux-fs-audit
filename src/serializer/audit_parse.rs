use crate::serializer::{Operation, OperationKey};
use crate::{map_of_values, reduce_equals_sign, split_key_val, unreferenced};
use regex::Regex;
use snailquote::unescape;
use std::collections::HashMap;
use std::error::Error;
use std::string::ToString;

const UNKNOWN_FIELD: &'static str = "unknown";

const USERNAME_KEY: &'static str = "UID";

const GROUP_KEY: &'static str = "GID";

const EXECUTABLE_KEY: &'static str = "exe";

const SYSCALL_KEY: &'static str = "SYSCALL";

const OPERATION_KEY: &'static str = "key";
const TIMESTAMP_KEY: &'static str = "msg";

impl Operation {
    pub(crate) fn new(log_output: String) -> Option<Self> {
        let values_map = map_of_values!(log_output);
        if values_map.contains_key(OPERATION_KEY) {
            return Some(Self {
                user: unescape(
                    values_map
                        .get(USERNAME_KEY)
                        .or(Some(&UNKNOWN_FIELD.to_string()))
                        .unwrap(),
                )
                .unwrap()
                .to_string(),
                group: unescape(
                    values_map
                        .get(GROUP_KEY)
                        .or(Some(&UNKNOWN_FIELD.to_string()))
                        .unwrap(),
                )
                .unwrap()
                .to_string(),
                executable: unescape(
                    values_map
                        .get(EXECUTABLE_KEY)
                        .or(Some(&UNKNOWN_FIELD.to_string()))
                        .unwrap(),
                )
                .unwrap()
                .to_string(),
                syscall: unescape(
                    values_map
                        .get(SYSCALL_KEY)
                        .or(Some(&UNKNOWN_FIELD.to_string()))
                        .unwrap(),
                )
                .unwrap()
                .to_string(),
                timestamp: LogParsingUtils::get_unix_time_from_timestamp(
                    values_map.get(TIMESTAMP_KEY).unwrap().to_string(),
                )
                .unwrap(),
                key: LogParsingUtils::get_operation_from_key(
                    values_map
                        .get(OPERATION_KEY)
                        .or(Some(&UNKNOWN_FIELD.to_string()))
                        .unwrap()
                        .to_string(),
                ),
            });
        }
        return None;
    }
}

struct LogParsingUtils {}

impl LogParsingUtils {
    fn create_a_map_of_values(coded_data: String) -> HashMap<String, String> {
        return coded_data
            .split(" ")
            .into_iter()
            .map(|unsplit_pair| split_key_val!(unsplit_pair))
            .filter(|tuple_of_str| !tuple_of_str.1.is_empty())
            .map(|tuple_of_str| reduce_equals_sign!(tuple_of_str))
            .map(|tuple_of_str| unreferenced!(tuple_of_str))
            .collect::<HashMap<_, _>>();
    }
    fn split_by_key_and_value(unsplit_pair: &str) -> (&str, &str) {
        unsplit_pair.split_at(unsplit_pair.find('=').or(Option::from(0)).unwrap())
    }

    fn reduce_equal_signs(x: &str) -> &str {
        return &x[x.find('=').or(Option::from(0)).unwrap() + 1..];
    }

    fn get_operation_from_key(operation_str: String) -> OperationKey {
        match operation_str.contains(&OperationKey::READ.to_string()) {
            true => OperationKey::READ,
            false => OperationKey::WRITE,
        }
    }
    fn get_unix_time_from_timestamp(msg_str: String) -> Result<String, Box<dyn Error>> {
        let unix_time_capture_regex = Regex::new(r"^audit\((\d+)\.\d+:\d+\):$")?;
        let Some(captured_values) = unix_time_capture_regex.captures(msg_str.as_str()) else {
            return Err(Box::try_from(regex::Error::Syntax(
                "Log line not compliant with the usual auditd format.".to_string(),
            ))?);
        };
        return Ok(captured_values
            .iter()
            .last()
            .unwrap()
            .unwrap()
            .as_str()
            .to_string());
    }
}

mod parser_macros {
    #[macro_export]
    macro_rules! map_of_values {
        ($x:ident) => {
            LogParsingUtils::create_a_map_of_values($x)
        };
    }
    #[macro_export]
    macro_rules! split_key_val {
        ($x:ident) => {
            LogParsingUtils::split_by_key_and_value($x)
        };
    }
    #[macro_export]
    macro_rules! reduce_equals_sign {
        ($x:tt) => {
            ($x.0, LogParsingUtils::reduce_equal_signs($x.1))
        };
    }
    #[macro_export]
    macro_rules! unreferenced {
        ($x:tt) => {
            (String::from($x.0), String::from($x.1))
        };
    }
    #[macro_export]
    macro_rules! get_key_from_op {
        ($x:tt) => {
            LogParsingUtils::get_operation_from_key($x)
        };
    }
    #[macro_export]
    macro_rules! timestamp {
        ($x:tt) => {
            LogParsingUtils::get_unix_time_from_timestamp($x)
        };
    }
}

#[cfg(test)]
mod test {
    use crate::serializer::audit_parse::{LogParsingUtils, UNKNOWN_FIELD};
    use crate::serializer::{Operation, OperationKey};
    use crate::{get_key_from_op, map_of_values, reduce_equals_sign, split_key_val, timestamp};
    use std::collections::HashMap;

    const COMPLIANT_LOG_LINE: &str = "type=SYSCALL msg=audit(1698576562.955:570): arch=c000003e syscall=257 success=yes exit=3 a0=ffffff9c a1=55a917750550 a2=90800 a3=0 items=1 ppid=20120 pid=20680 auid=1000 uid=1000 gid=1000 euid=1000 suid=1000 fsuid=1000 egid=1000 sgid=1000 fsgid=1000 tty=pts2 ses=14 comm=\"ls\" exe=\"/usr/bin/ls\" subj=unconfined key=\"READ\"ARCH=x86_64 AUID=\"maciek\" UID=\"maciek\" GID=\"maciek\" EUID=\"maciek\" SUID=\"maciek\" FSUID=\"maciek\" EGID=\"maciek\" SGID=\"maciek\"";
    const NUMBER_OF_SPACES_IN_LINE: usize = 36;

    #[test]
    fn should_create_a_map_of_values() {
        //given
        let input = String::from(COMPLIANT_LOG_LINE);
        //when
        let map: HashMap<String, String> = map_of_values!(input);
        //then
        assert_eq!(map.len(), NUMBER_OF_SPACES_IN_LINE);
    }

    #[test]
    fn should_split_by_key_and_value() {
        //given
        let input = "type=SYSCALL";
        //when
        let split_values = split_key_val!(input);
        //then
        assert_eq!(split_values.0, "type");
        assert_eq!(split_values.1, "=SYSCALL");
    }

    #[test]
    fn should_reduce_equal_sign() {
        //given
        let input = ("type", "=SYSCALL");
        //when
        let split_values: (&str, &str) = reduce_equals_sign!(input);
        //then
        assert_eq!(split_values.0, "type");
        assert_eq!(split_values.1, "SYSCALL");
    }

    #[test]
    fn should_deduce_correct_operation() {
        //given
        let input = "\"READ\"ARCH=x86_64".to_string();
        //when
        let op = get_key_from_op!(input);
        //then
        matches!(op, OperationKey::READ);
    }

    #[test]
    fn should_have_one_of_fields_as_unknown() {
        //given
        let input = String::from(COMPLIANT_LOG_LINE);
        //when
        let operation = Operation::new(input);
        //then
        assert_eq!(operation.unwrap().syscall, UNKNOWN_FIELD);
    }
    #[test]
    fn should_extract_timestamp() {
        //give
        let expected_timestamp = "1698576562".to_string();
        //when
        let timestamp_string = "audit(1698576562.955:570):".to_string();
        let result = timestamp!(timestamp_string).unwrap();
        //then
        assert_eq!(result, expected_timestamp);
    }
}
