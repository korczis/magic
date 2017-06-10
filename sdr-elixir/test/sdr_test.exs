defmodule SdrTest do
  use ExUnit.Case
  doctest Sdr

  @on_load {:init, 0}
  @not_loaded :nif_lib_not_loaded

  def init do
    path = :filelib.wildcard('../target/{debug,release}/libsdr.*')
    |> hd
    |> :filename.rootname

    :ok = :erlang.load_nif(path, 0)

#    IO.inspect add(5, 2)
#    try do
#      panic_test()
#    rescue
#      a in ErlangError -> IO.inspect a
#    end
#    IO.inspect struct_argument(%TestStruct{})
#
#    thing = make_resource_struct
#    IO.inspect read_resource_struct(thing)
#
#    IO.inspect string_test

    :ok
  end

  test "the truth" do
    assert 1 + 1 == 2
  end

   def add(_a, _b), do: exit(@not_loaded)
end
