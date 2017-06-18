defmodule SdrTest do
  use ExUnit.Case
  doctest Sdr

  test "add" do
    assert Sdr.Native.add(1, 2) == {:ok, 3}
  end

  test "sub" do
    assert Sdr.Native.sub(3, 2) == {:ok, 1}
  end
end
