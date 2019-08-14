import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onSubmit)
import Http exposing (..)
import Json.Decode as Decode exposing (..)
import Json.Encode as Encode exposing (..)

import Bootstrap.CDN as CDN
import Bootstrap.Grid as Grid
import Bootstrap.Grid.Row as Row
import Bootstrap.Grid.Col as Col
import Bootstrap.Form as Form
import Bootstrap.Form.Input as Input
import Bootstrap.Button as Button
import Bootstrap.Text as Text

main = 
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }

-- MODEL

type alias Model = { log : List (String, String), input : String }

-- UPDATE

type Msg = 
    Clear
  | GotCalc (Result Http.Error String)
  | SetCalc String
  | SubmitForm

init : () -> (Model, Cmd Msg)
init _ = 
  (Model [] "", Cmd.none)

subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none

calculate : String -> Cmd Msg
calculate input =
  Http.post
    { url = "http://localhost:8001/calculate"
    , body = Http.jsonBody (calcEncoder input)
    , expect = Http.expectJson GotCalc calcDecoder
    }

calcEncoder : String -> Encode.Value
calcEncoder calc =
  Encode.object
    [ ("calc", Encode.string calc) ]

calcDecoder : Decoder String
calcDecoder = 
  Decode.string

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Clear  ->
      ((Model [] ""), Cmd.none)
    GotCalc (Ok val) ->
      ((Model (model.log ++ [(model.input, val)]) ""), Cmd.none)
    GotCalc (Err _) ->
      ((Model (model.log) ""), Cmd.none)
    SetCalc input ->
      ({ model | input = input }, Cmd.none)
    SubmitForm -> 
      (model, calculate model.input)

-- VIEW

calcForm : Model -> Html Msg
calcForm model = 
  Form.formInline
    [ onSubmit SubmitForm
    ]
    [ Form.group []
       -- [ Form.label [] [ text "Calculation" ]
      [ Input.text [ Input.attrs [ placeholder "Calculation", onInput SetCalc ] ]
      ]
    , Button.button [ Button.primary ] [ text "Calculate" ]
    ]

view : Model -> Html Msg
view model =
  Grid.container [ class "center" ]
    [ h2 [ class "text-center" ] [ text "Rustulator" ]
    , Grid.container [ class "border" ] <| List.concatMap (\(i, v) -> 
      [ Grid.row [] 
        [ Grid.col [ Col.xs6, Col.textAlign Text.alignXsLeft  ] [ text i ] 
        , Grid.col [ Col.xs6, Col.textAlign Text.alignXsRight ] [ text ("= " ++ v) ] 
        ]
      ]) model.log
    , Grid.row [ Row.centerXs ] [ Grid.col [ Col.xsAuto ] [ calcForm model ] ]
    ]
