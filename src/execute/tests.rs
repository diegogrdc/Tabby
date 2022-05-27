#[cfg(test)]
mod tests_execute {
    use crate::create_vir_mach_from_lines;
    use crate::get_lines;

    #[test]
    fn test_print_ok_1() {
        // Setup
        let filename = "print_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(
            vir_mach.output,
            format!("Hello World!\n1 2 3\n-10 0.99 True End\n")
        );
    }

    #[test]
    fn test_arithmetic_ok_1() {
        // Setup
        let filename = "arithmetic_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(
            vir_mach.output,
            format!("3, 60, 4, -9\n2.83, 0.375, 2.6, -0.33\n")
        );
    }

    #[test]
    fn test_comps_ok_1() {
        // Setup
        let filename = "comps_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(
            vir_mach.output,
            format!("True, True, True, False, True, True\n")
        );
    }

    #[test]
    fn test_boolean_ops_ok_1() {
        // Setup
        let filename = "boolean_ops_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(
            vir_mach.output,
            format!("False, True, False, True, False\n")
        );
    }

    #[test]
    fn test_if_ok_1() {
        // Setup
        let filename = "if_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("In If\nAfter If\n"));
    }

    #[test]
    fn test_if_ok_2() {
        // Setup
        let filename = "if_ok_2".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("After If\n"));
    }

    #[test]
    fn test_if_else_ok_1() {
        // Setup
        let filename = "if_else_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("Positive\n"));
    }

    #[test]
    fn test_if_else_ok_2() {
        // Setup
        let filename = "if_else_ok_2".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("Zero\n"));
    }

    #[test]
    fn test_if_else_ok_3() {
        // Setup
        let filename = "if_else_ok_3".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("Negative\n"));
    }

    #[test]
    fn test_while_ok_1() {
        // Setup
        let filename = "while_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("1, 2, 3, 4, 5, 6, 7, 8, 9, 10\n"));
    }

    #[test]
    fn test_for_ok_1() {
        // Setup
        let filename = "for_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(
            vir_mach.output,
            format!("1->2->4->8->16->32->64->128->256->512->1024->2048")
        );
    }

    #[test]
    fn test_assign_ok_1() {
        // Setup
        let filename = "assign_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("3 1.1 True\n"));
    }

    #[test]
    fn test_arrays_ok_1() {
        // Setup
        let filename = "arrays_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("1, 4, 9, 16, 25\n"));
    }

    #[test]
    fn test_mats_ok_1() {
        // Setup
        let filename = "mats_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("3 6 9 \n2 5 8 \n1 4 7 \n"));
    }

    #[test]
    fn test_fns_ok_1() {
        // Setup
        let filename = "fns_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("4 0.192 True\nVoid!"));
    }

    #[test]
    fn test_fns_ok_2() {
        // Setup
        let filename = "fns_ok_2".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("0 0 False\n"));
    }

    #[test]
    fn test_fns_params_ok_1() {
        // Setup
        let filename = "fns_params_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(
            vir_mach.output,
            format!("05True\n4.2False5\nTrue62\n00False\n0False0\nFalse00\n00False\n")
        );
    }

    #[test]
    fn test_fns_recursive_ok_1() {
        // Setup
        let filename = "fns_recursive_ok_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("1, 1, 2, 3, 5, 8, 13, 21"));
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_fail_1() {
        // Setup
        let filename = "out_of_bounds_fail_1".to_string();
        let path = format!("./tests_execute/{}.tabbyic", filename);
        let lines = get_lines(&path, &filename);
        let mut vir_mach = create_vir_mach_from_lines(lines);
        vir_mach.execute();
        // Assert output
        assert_eq!(vir_mach.output, format!("1, 1, 2, 3, 5, 8, 13, 21"));
    }
}
