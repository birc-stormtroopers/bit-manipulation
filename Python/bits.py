x = 0xf4e2
print(f"{x:016b}")

x = 0xf4e2  # [f: 1111, 4: 0010, e: 1110, 2: 0010]

print(f"x:                        {x:016b}")
print(f"x shifted left by two:  {x << 2:016b}")  # doesn't stay in 16 bits
print(f"x shifted right by two:   {(x >> 2):016b}")
print("")

print(f"x:                        {x:016b}")
print(f"x >> 2:                   {x >> 2:016b}")
print(f"x & (x >> 2):             {x & (x >> 2):016b}")
print("")

print(f"x:                        {x:016b}")
print(f"x << 2:                 {x << 2:016b}")        # 18-bit word now
print(f"x | (x << 2):           {x | (x << 2):016b}")  # still 18-bit word
print("")

print(f"x:                        {x:16b}")
print(f"x << 2:                 {x << 2:16b}")
print(f"x ^ (x << 2):           {x ^ (x << 2):16b}")
print("")

print(f"x:                        {x:16b}")
print(f"~x:                      {~x:16b}")  # -(x + 1) ???
