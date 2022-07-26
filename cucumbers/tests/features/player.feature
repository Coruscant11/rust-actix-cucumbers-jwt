@serial
Feature: Player

    @serial
    Scenario: Register player not already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567890 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        When I register the player
        Then I receive a code 200
        And the player is registered

    @serial
    Scenario: Register a player already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567891 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I register the player
        Then I receive a code 400

    @serial
    Scenario: Register a player with invalid data
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | letters    | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        And the discord_id is invalid
        When I register the player
        Then I receive a code 400
        And the player is not registered

    @serial
    Scenario: Get a player not registered yet
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567892 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        When I get the player
        Then I receive a code 404

    @serial
    Scenario: Get a player already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567893 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I get the player
        Then I receive a code 200
        And the player is returned

    @serial
    Scenario: Get all players
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567894 | Player | 111222333 | 333222111 |
            | 0987654325 | Casual | 444555666 | 666554444 |
        And the discord_id is already registered
        When I get all players
        Then I receive a code 200
        And the players are returned

    @serial
    Scenario: Update a player not registered yet
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567896 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        When I update the player
            | discord_id | name | na_id     | jp_id     |
            | 1234567896 | Jp   | 111222333 | 333222111 |
        Then I receive a code 404

    @serial
    Scenario: Update a player already registered
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567897 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I update the player
            | discord_id | name | na_id     | jp_id     |
            | 1234567897 | Jp   | 999888777 | 333222111 |
        Then I receive a code 200
        And the player is updated
            | discord_id | name | na_id     | jp_id     |
            | 1234567897 | Jp   | 999888777 | 333222111 |

    @serial
    Scenario: Update a player with invalid data
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567898 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I update the player
            | discord_id      | name | na_id     | jp_id     |
            | invalidUpdateId | Jp   | 111222333 | 333222111 |
        Then I receive a code 400

    @serial
    Scenario: Delete a player not registered yet
        Given a player
            | discord_id | name   | na_id     | jp_id     |
            | 1234567899 | Player | 111222333 | 333222111 |
        And the discord_id is not already registered
        When I delete the player
        Then I receive a code 404

    @serial
    Scenario: Delete a player already registered
        Given a player
            | discord_id  | name   | na_id     | jp_id     |
            | 12345678910 | Player | 111222333 | 333222111 |
        And the discord_id is already registered
        When I delete the player
        Then I receive a code 200
        And the player is deleted