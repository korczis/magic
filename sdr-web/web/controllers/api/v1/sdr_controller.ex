defmodule SdrWeb.API.V1.SdrController do
  use SdrWeb.Web, :controller

  require Logger

  def list(conn, _params) do
    conn
    |> json([])
  end
end
