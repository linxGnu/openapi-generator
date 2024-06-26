/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * API version: 1.0.0
 * Generated by: OpenAPI Generator (https://openapi-generator.tech)
 */

package petstoreserver

import (
	"github.com/gin-gonic/gin"
)

type UserAPI interface {


    // CreateUser Post /v2/user
    // Create user 
     CreateUser(c *gin.Context)

    // CreateUsersWithArrayInput Post /v2/user/createWithArray
    // Creates list of users with given input array 
     CreateUsersWithArrayInput(c *gin.Context)

    // CreateUsersWithListInput Post /v2/user/createWithList
    // Creates list of users with given input array 
     CreateUsersWithListInput(c *gin.Context)

    // DeleteUser Delete /v2/user/:username
    // Delete user 
     DeleteUser(c *gin.Context)

    // GetUserByName Get /v2/user/:username
    // Get user by user name 
     GetUserByName(c *gin.Context)

    // LoginUser Get /v2/user/login
    // Logs user into the system 
     LoginUser(c *gin.Context)

    // LogoutUser Get /v2/user/logout
    // Logs out current logged in user session 
     LogoutUser(c *gin.Context)

    // UpdateUser Put /v2/user/:username
    // Updated user 
     UpdateUser(c *gin.Context)

}