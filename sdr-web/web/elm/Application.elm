module Application exposing (..)

import Bootstrap.Alert as Alert
import Bootstrap.Button as Button
import Bootstrap.Form.Input as Input
import Bootstrap.Grid as Grid
import Bootstrap.Navbar as Navbar
import FontAwesome.Web as Icon
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Navigation
import Page.Home

import Msg

-- MODEL

type alias Model =
    { history : List Navigation.Location
    , navbarState : Navbar.State
    , counter : Int
    }



-- INIT

init : Navigation.Location -> ( Model, Cmd Msg.Msg )
init location =
    let
        ( navbarState, navbarCmd ) = Navbar.initialState Msg.NavbarMsg
    in
        ( { history = [ location ], counter = 0, navbarState = navbarState }, navbarCmd )



-- UPDATE

update : Msg.Msg -> Model -> ( Model, Cmd Msg.Msg )
update message model =
    case message of
        Msg.Dec ->
            { model | counter = model.counter - 1 } ! []

        Msg.Inc ->
            { model | counter = model.counter + 1 } ! []

        Msg.NavbarMsg state ->
            { model | navbarState = state } ! []

        Msg.UrlChange location ->
            ( { model | history = location :: model.history }
            , Cmd.none
            )



-- VIEW

navbar : Model -> Html Msg.Msg
navbar model =
    div []
        [ Navbar.config Msg.NavbarMsg
            |> Navbar.withAnimation
            |> Navbar.brand [ href "#" ] [ text "MagicSense" ]
            |> Navbar.items
                [ Navbar.itemLink [ href "/#/radios" ] [ text "Radios" ]
                , Navbar.itemLink [ href "/#/map" ] [ text "Map" ]
                ]
            |> Navbar.customItems
                -- Add custom items
                [ Navbar.formItem []
                    [ Input.text [ Input.attrs <| [ placeholder "enter" ] ]
                    , Button.button
                        [ Button.success
                        , Button.attrs [ class "ml-sm-2" ]
                        ]
                        [ text "Search" ]
                    ]
                , Navbar.textItem [ class "muted ml-sm-2" ] [ text "Sign in" ]
                , Navbar.textItem [ class "muted ml-sm-2" ] [ text "Sign up" ]
                ]
            |> Navbar.view model.navbarState
        ]


mainContent : Model -> Html Msg.Msg
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
            , onClick Msg.Dec
            ]
            [ text "- 1" ]
        , span
            []
            [ text <| toString model.counter ]
        , button
            [ class "btn btn-primary"
            , onClick Msg.Inc
            ]
            [ text "+ 1" ]
        ]


view : Model -> Html Msg.Msg
view model =
    div []
        [ navbar model -- Interactive and responsive menu

        -- , mainContent model
        ]



-- If you use animations as above or you use dropdowns in your navbar you need to configure subscriptions too


subscriptions : Model -> Sub Msg.Msg
subscriptions model =
    Navbar.subscriptions model.navbarState Msg.NavbarMsg


viewLink : String -> Html msg
viewLink name =
    li [] [ a [ href ("#" ++ name) ] [ text name ] ]


viewLocation : Navigation.Location -> Html msg
viewLocation location =
    li [] [ text (location.pathname ++ location.hash) ]
