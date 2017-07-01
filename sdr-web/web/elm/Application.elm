module Application exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)

import Bootstrap.Alert as Alert
import Bootstrap.Grid as Grid
import Bootstrap.Navbar as Navbar

-- MODEL

type alias Model =
    { counter : Int
    , navbarState : Navbar.State
    }


init : ( Model, Cmd Msg )
init =
    let
        (navbarState, navbarCmd) = Navbar.initialState NavbarMsg
    in
        ({ counter = 0, navbarState = navbarState }, navbarCmd )



-- UPDATE

type Msg
    = Inc
    | Dec
    | NavbarMsg Navbar.State


update : Msg -> Model -> ( Model, Cmd Msg )
update message model =
    case message of
        Dec ->
            { model | counter = model.counter - 1 } ! []
        Inc ->
            { model | counter = model.counter + 1 } ! []
        NavbarMsg state ->
            { model | navbarState = state } ! []


-- VIEW

navbar : Model -> Html Msg
navbar model =
    div []
    [ Navbar.config NavbarMsg
        |> Navbar.withAnimation
        |> Navbar.brand [ href "#"] [ text "Brand"]
        |> Navbar.items
            [ Navbar.itemLink [href "#"] [ text "Item 1"]
            , Navbar.itemLink [href "#"] [ text "Item 2"]
            ]
        |> Navbar.view model.navbarState
    ]



mainContent : Model -> Html Msg
mainContent model =
    div
        [ class "container" ]
        [ h1
            []
            [ img [ src "/images/phoenix.png" ] []
            , text "Hot loading example!"
            ]
        , button
            [ class "btn btn-primary"
            , onClick Dec
            ]
            [ text "- 1" ]
        , span
            []
            [ text <| toString model.counter ]
        , button
            [ class "btn btn-primary"
            , onClick Inc
            ]
            [ text "+ 1" ]
        ]

view : Model -> Html Msg
view model =
    div []
        [ navbar model        -- Interactive and responsive menu
        -- , mainContent model
        ]

-- If you use animations as above or you use dropdowns in your navbar you need to configure subscriptions too

subscriptions : Model -> Sub Msg
subscriptions model =
    Navbar.subscriptions model.navbarState NavbarMsg
