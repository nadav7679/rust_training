import os
import time
import random

def run_times(args):
    t1_rust = time.time()
    # os.system(f"cargo run -q {args}") # Results in longet computation than python. Maybe it recompile every time?
    os.system(f"target/debug/rolling_avg {args}")
    rtime = time.time() - t1_rust
    
    t1_python = time.time()
    os.system(f"python3.10 python_avg.py {args}")
    ptime = time.time() - t1_python
    
    return rtime, ptime


def main():
    N = 100
    rust_times = []
    python_times = []
    
    for _ in range(N):
        nums = [str(random.randint(-10000, 10000)) for _ in range(random.randint(100, 1000))]
        k = random.randint(1, 100)
        
        args_str = f"{k} {' '.join(nums)}"
        
        rtime, ptime = run_times(args_str)
        rust_times.append(rtime)
        python_times.append(ptime)
        
    print(f"For {N} samples:\n \
          Rust average runtime: {sum(rust_times)/N:.04}\n \
          Python average runtime: {sum(python_times)/N:.04}\n \
          Rust is {sum(python_times)/sum(rust_times)} faster than Python!")
    
    # --- Result example --- #
    #
    # For 100 samples:
    # Rust average runtime: 0.001603
    # Python average runtime: 0.02741
    # Rust is 17.093340094306654 faster than Python!
    # 
    # Success!!

    
if __name__ == "__main__":
    main()