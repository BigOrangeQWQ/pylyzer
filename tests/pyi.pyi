import typing

x: typing.Any

def f(x: int, y: int) -> int: ...

class C:
    x: int
    y: int
    def __init__(self, x: int): ...
    def f(self, x: int) -> int: ...

def g[T: C](x: T) -> T: ...
