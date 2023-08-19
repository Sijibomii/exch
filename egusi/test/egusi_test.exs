defmodule EgusiTest do
  use ExUnit.Case
  doctest Egusi

  test "greets the world" do
    assert Egusi.hello() == :world
  end
end
