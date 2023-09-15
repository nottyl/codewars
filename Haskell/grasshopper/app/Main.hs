module GameMove where

-- move takes two Int arguments and return one Int result
move :: Int -> Int -> Int

-- Logistics: if move(3, 6) and output = 15, that means the second number are multiplied by two and then adding on the first number.
move x y = x + (y * 2)
