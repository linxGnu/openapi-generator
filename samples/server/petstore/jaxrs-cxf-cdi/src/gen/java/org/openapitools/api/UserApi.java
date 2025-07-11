package org.openapitools.api;

import org.openapitools.model.User;
import org.openapitools.api.UserApiService;

import javax.ws.rs.*;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.Response;
import javax.ws.rs.core.SecurityContext;
import javax.enterprise.context.RequestScoped;
import javax.inject.Inject;

import io.swagger.annotations.*;
import java.io.InputStream;

import org.apache.cxf.jaxrs.ext.PATCH;
import org.apache.cxf.jaxrs.ext.multipart.Attachment;
import org.apache.cxf.jaxrs.ext.multipart.Multipart;

import java.util.Map;
import java.util.List;
import javax.validation.constraints.*;
import javax.validation.Valid;
@Path("/user")
@RequestScoped

@Api(description = "the user API")


@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaJAXRSCXFCDIServerCodegen", comments = "Generator version: 7.15.0-SNAPSHOT")

public class UserApi  {

  @Context SecurityContext securityContext;

  @Inject UserApiService delegate;


    @POST
    
    @Consumes({ "application/json" })
    
    @ApiOperation(value = "Create user", notes = "This can only be done by the logged in user.", response = Void.class, authorizations = {
        
        @Authorization(value = "api_key")
         }, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class) })
    public Response createUser(@ApiParam(value = "Created user object" ,required=true) User user) {
        return delegate.createUser(user, securityContext);
    }

    @POST
    @Path("/createWithArray")
    @Consumes({ "application/json" })
    
    @ApiOperation(value = "Creates list of users with given input array", notes = "", response = Void.class, authorizations = {
        
        @Authorization(value = "api_key")
         }, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class) })
    public Response createUsersWithArrayInput(@ApiParam(value = "List of user object" ,required=true) List<@Valid User> user) {
        return delegate.createUsersWithArrayInput(user, securityContext);
    }

    @POST
    @Path("/createWithList")
    @Consumes({ "application/json" })
    
    @ApiOperation(value = "Creates list of users with given input array", notes = "", response = Void.class, authorizations = {
        
        @Authorization(value = "api_key")
         }, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class) })
    public Response createUsersWithListInput(@ApiParam(value = "List of user object" ,required=true) List<@Valid User> user) {
        return delegate.createUsersWithListInput(user, securityContext);
    }

    @DELETE
    @Path("/{username}")
    
    
    @ApiOperation(value = "Delete user", notes = "This can only be done by the logged in user.", response = Void.class, authorizations = {
        
        @Authorization(value = "api_key")
         }, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 400, message = "Invalid username supplied", response = Void.class),
        @ApiResponse(code = 404, message = "User not found", response = Void.class) })
    public Response deleteUser(@ApiParam(value = "The name that needs to be deleted",required=true) @PathParam("username") String username) {
        return delegate.deleteUser(username, securityContext);
    }

    @GET
    @Path("/{username}")
    
    @Produces({ "application/xml", "application/json" })
    @ApiOperation(value = "Get user by user name", notes = "", response = User.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = User.class),
        @ApiResponse(code = 400, message = "Invalid username supplied", response = Void.class),
        @ApiResponse(code = 404, message = "User not found", response = Void.class) })
    public Response getUserByName(@ApiParam(value = "The name that needs to be fetched. Use user1 for testing.",required=true) @PathParam("username") String username) {
        return delegate.getUserByName(username, securityContext);
    }

    @GET
    @Path("/login")
    
    @Produces({ "application/xml", "application/json" })
    @ApiOperation(value = "Logs user into the system", notes = "", response = String.class, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = String.class),
        @ApiResponse(code = 400, message = "Invalid username/password supplied", response = Void.class) })
    public Response loginUser( @NotNull  @Pattern(regexp="^[a-zA-Z0-9]+[a-zA-Z0-9\\.\\-_]*[a-zA-Z0-9]+$")@ApiParam(value = "The user name for login",required=true)  @QueryParam("username") String username,  @NotNull @ApiParam(value = "The password for login in clear text",required=true)  @QueryParam("password") String password) {
        return delegate.loginUser(username, password, securityContext);
    }

    @GET
    @Path("/logout")
    
    
    @ApiOperation(value = "Logs out current logged in user session", notes = "", response = Void.class, authorizations = {
        
        @Authorization(value = "api_key")
         }, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 200, message = "successful operation", response = Void.class) })
    public Response logoutUser() {
        return delegate.logoutUser(securityContext);
    }

    @PUT
    @Path("/{username}")
    @Consumes({ "application/json" })
    
    @ApiOperation(value = "Updated user", notes = "This can only be done by the logged in user.", response = Void.class, authorizations = {
        
        @Authorization(value = "api_key")
         }, tags={ "user" })
    @ApiResponses(value = { 
        @ApiResponse(code = 400, message = "Invalid user supplied", response = Void.class),
        @ApiResponse(code = 404, message = "User not found", response = Void.class) })
    public Response updateUser(@ApiParam(value = "name that need to be deleted",required=true) @PathParam("username") String username, @ApiParam(value = "Updated user object" ,required=true) User user) {
        return delegate.updateUser(username, user, securityContext);
    }
}
