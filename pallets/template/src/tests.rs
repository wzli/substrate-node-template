use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

fn get_event() -> Option<crate::Event<Test>> {
	match System::events().pop()?.event {
		Event::TemplateModule(e) => Some(e),
		_ => None,
	}
}

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn insert_and_get_something() {
	new_test_ext().execute_with(|| {
		let test_val = 500;
		// insert test value
		assert_ok!(TemplateModule::insert(Origin::signed(1), test_val));
		// retreive key from the success event
		let key = match get_event().unwrap() {
			crate::Event::<Test>::SomethingInserted(x, _) => x,
			_ => panic!("wrong event expected"),
		};
		// retrieve the previously inserted value
		assert_eq!(TemplateModule::tuple_map(key).unwrap(), test_val);
	});
}
