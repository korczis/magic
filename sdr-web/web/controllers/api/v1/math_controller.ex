defmodule SdrWeb.API.V1.MathController do
  use SdrWeb.Web, :controller

  require Logger

  def calculate(conn, params) do
    Logger.info(params)

    conn
    |> json(%{msg: "Not yet implemented!"})
  end
end
