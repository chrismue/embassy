use embassy_futures::{block_on, yield_now};

#[test]
fn test_yield_now_completes() {
    block_on(async {
        // Test that yield_now completes after yielding once
        yield_now().await;
    });
}

#[test]
fn test_yield_now_in_loop() {
    block_on(async {
        let mut count = 0;
        
        // Test that yield_now works correctly in a loop
        for _ in 0..5 {
            count += 1;
            yield_now().await;
        }
        
        assert_eq!(count, 5);
    });
}

#[test]
fn test_multiple_yields() {
    block_on(async {
        // Test multiple sequential yields
        yield_now().await;
        yield_now().await;
        yield_now().await;
    });
}
