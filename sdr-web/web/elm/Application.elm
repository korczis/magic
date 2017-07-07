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
import Json.Decode as Decode exposing (Value)
import Navigation
import Assets
import Data.Session as Session
import Data.User as User
import Msg
import Model exposing (Model)
import Page
import Page.Home
import Page.Map
import Ports
import Route
import Util exposing ((=>))


-- INIT


init : Value -> Navigation.Location -> ( Model, Cmd Msg.Msg )
init value location =
    let
        ( navbarState, navbarCmd ) =
            Navbar.initialState Msg.NavbarMsg
    in
        ( { history = [ location ]
          , counter = 0
          , navbar =
                { state = navbarState
                }
          , page = Page.Loaded Page.initialPage
          , session = Session.Session Nothing
          }
        , Cmd.batch [ navbarCmd ]
        )



-- UPDATE


update : Msg.Msg -> Model -> ( Model, Cmd Msg.Msg )
update msg model =
    updatePage (Page.getPage model.page) msg model


updatePage : Page.Page -> Msg.Msg -> Model -> ( Model, Cmd Msg.Msg )
updatePage page msg model =
    let
        session =
            model.session

        toPage toModel toMsg subUpdate subMsg subModel =
            let
                ( newModel, newCmd ) =
                    subUpdate subMsg subModel
            in
                ( { model | page = Page.Loaded (toModel newModel) }, Cmd.map toMsg newCmd )

        -- errored =  pageErrored model
    in
        case ( msg, page ) of
            ( Msg.Dec, _ ) ->
                { model | counter = model.counter - 1 } ! []

            ( Msg.Inc, _ ) ->
                { model | counter = model.counter + 1 } ! []

            ( Msg.NavbarMsg state, _ ) ->
                let
                    modelNavbar =
                        model.navbar
                in
                    { model | navbar = { modelNavbar | state = state } } ! []

            ( Msg.UrlChange location, _ ) ->
                ( { model | history = location :: model.history }
                , Cmd.none
                )

            ( Msg.SetRoute route, _ ) ->
                ( { model | page = Page.Loaded Page.Home }, Cmd.none )

            ( Msg.SetUser user, _ ) ->
                let
                    session =
                        model.session

                    cmd =
                        -- If we just signed out, then redirect to Home.
                        if session.user /= Nothing && user == Nothing then
                            Route.modifyUrl Route.Home
                        else
                            Cmd.none
                in
                    { model | session = { session | user = user } }
                        => cmd



-- VIEW


navbar : Model -> Html Msg.Msg
navbar model =
    div []
        [ Navbar.config Msg.NavbarMsg
            |> Navbar.withAnimation
            |> Navbar.brand
                [ href "/#/" ]
                [ img
                    [ src (Assets.path <| Assets.star)
                    , class "d-inline-block"
                    , style
                        [ ( "width", "30px" )
                        , ( "margin-right", "15px" )
                        ]
                    ]
                    []
                , text "MagicSense"
                ]
            |> Navbar.items
                [ Navbar.itemLink [ href "/#/radio" ] [ text "Radio" ]
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
            |> Navbar.view model.navbar.state
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


viewPage : Page.State -> Html Msg.Msg
viewPage page =
    case page of
        Page.Loaded Page.Blank ->
            div [] [ text "Blank" ]

        Page.Loaded Page.Home ->
            Page.Home.view

        Page.Loaded Page.Map ->
            Page.Map.view

        _ ->
            div [] []


view : Model -> Html Msg.Msg
view model =
    div []
        [ navbar model -- Interactive and responsive menu
        , viewPage model.page

        -- , mainContent model
        ]



-- If you use animations as above or you use dropdowns in your navbar you need to configure subscriptions too


subscriptions : Model -> Sub Msg.Msg
subscriptions model =
    Sub.batch
        [ Navbar.subscriptions model.navbar.state Msg.NavbarMsg
        , Sub.map Msg.SetUser sessionChange
        ]


sessionChange : Sub (Maybe User.User)
sessionChange =
    Ports.onSessionChange (Decode.decodeValue User.decoder >> Result.toMaybe)


viewLink : String -> Html msg
viewLink name =
    li [] [ a [ href ("#" ++ name) ] [ text name ] ]


viewLocation : Navigation.Location -> Html msg
viewLocation location =
    li [] [ text (location.pathname ++ location.hash) ]
