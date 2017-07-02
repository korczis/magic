module Page.Home exposing (..)

import Data.Session as Session exposing (Session)
import Http
import Page.Errored as Errored exposing (PageLoadError, pageLoadError)
import SelectList exposing (SelectList)
import Task exposing (Task)
import Views.Page as Page

-- MODEL --


type alias Model =
    {
    }


init : Session -> Task PageLoadError Model
init session =
    let
        feedSources =
            if session.user == Nothing then
                SelectList.singleton globalFeed
            else
                SelectList.fromLists [] yourFeed [ globalFeed ]

        loadTags =
            Request.Article.tags
                |> Http.toTask

        loadSources =
            Feed.init session feedSources

        handleLoadError _ =
            pageLoadError Page.Home "Homepage is currently unavailable."
    in
    Task.map Model loadTags
        |> Task.mapError handleLoadError

-- UPDATE --


type Msg
    = Default