#!/usr/bin/env python2

import unittest
import example

class TestExample(unittest.TestCase):
    def test(self):
        self.assertEqual(example.fun1(), "Hello!")
        self.assertEqual(example.fun2("xy"), "xyz")
        self.assertEqual(example.fun3(5, 10), 15)
        self.assertEqual(example.fun4([1, 2, 3, 4]), [2, 4, 6, 8])
        self.assertEqual(example.fun5([1, 2, 3, 4]), [2, 4, 6, 8])
        self.assertEqual(example.fun6({"a": 2, "b": 5}), 10)

if __name__ == '__main__':
    unittest.main()
