defmodule SdrWeb.Router do
  use SdrWeb.Web, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_flash
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/api/v1", as: :api_v1, alias: SdrWeb.API.V1 do
    pipe_through :api

    post "/math/calculate", MathController, :calculate

    get "/news/test", NewsController, :test

    get "/sdr/list", SdrController, :list
  end

  scope "/", SdrWeb do
    pipe_through :browser # Use the default browser stack

    get "/*path", PageController, :index
  end

  # Other scopes may use custom stacks.
  # scope "/api", SdrWeb do
  #   pipe_through :api
  # end
end
